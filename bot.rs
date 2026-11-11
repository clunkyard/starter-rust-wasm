// A starter robot for rules v2, in Rust on WebAssembly ABI 4: it calls the
// robot core through the clunkyard.* imports in clunkyard.rs, next to this
// file. It sweeps a medium scan to find a robot, narrows the scan to hold it,
// leads it with shots whose power suits the range, fires only when the
// turret is on target, keeps battery back for driving, looks again where it
// last saw one, follows a passive cue when it has nothing better and steps
// out of the storm.
//
// The trade-offs (the imports answer with the numbers the hello set):
//   scan     a narrow arc reaches further; a wide one finds robots nearby
//   power    heavy shots hit harder for less battery, but fly slowly
//   battery  shots and driving share it, and it refills slowly
//   turret   it turns at a limited rate; cy_gun_aim_now is where it points now
//   fuel     every import costs fuel, so read each fact once a turn
// A cue (cy_tracker_cue) says roughly where a robot is, within its
// bearing_error degrees: good for where to look, not where to shoot.
// Clunkyard WebAssembly ABI 4: the robot calls the core through imports
// from "clunkyard", declared in clunkyard.h. Export memory and, through
// CY_ROBOT(start, turn), clunkyard_start and clunkyard_turn. Each import
// returns a status (CY_OK 0, CY_NONE 1 for absent) and writes results
// through pointers; each costs fuel. In C each import is cy_<name>, as
// cy_sensors_nearest(&contact).
// match: match_mode match_ruleset match_control match_robots match_is_3d
//   match_tick_rate match_max_ticks robot_id robot_seed
// clock: clock_tick clock_seconds clock_remaining_ticks
// arena: arena_size arena_gravity arena_terrain_kind arena_ground_height
// me: me_health me_position me_velocity me_heading me_pitch me_roll
//   me_angular_velocity me_attitude me_grounded me_altitude me_applied
// battery: battery_charge battery_capacity battery_regen
//   battery_sustainable_drive battery_compute battery_drive_cost
// drive: drive_hover_lift drive_throttle drive_turn drive_pitch_turn
//   drive_thrust drive_strafe drive_lift drive_yaw_torque
//   drive_pitch_torque drive_roll_torque drive_brake drive_get_throttle
//   drive_get_turn drive_get_pitch_turn drive_get_thrust drive_get_strafe
//   drive_get_lift drive_get_yaw_torque drive_get_pitch_torque
//   drive_get_roll_torque drive_get_brake
// gun: gun_aim_now gun_cooldown_ticks gun_spread gun_max_power
//   gun_damage_scale gun_can_fire gun_shot_speed gun_shot_cost
//   gun_reload_ticks gun_shot_damage gun_power_for_speed
//   gun_power_for_flight_time gun_lead gun_lead_point gun_aim_heading
//   gun_aim_pitch gun_fire gun_power gun_get_aim_heading gun_get_aim_pitch
//   gun_get_fire gun_get_power
// scanner: scanner_last scanner_proximity_range scanner_reach
//   scanner_arc_for_reach scanner_heading scanner_arc scanner_pitch
//   scanner_get_heading scanner_get_arc scanner_get_pitch
// sensors: sensors_contacts sensors_nearest sensors_cues sensors_contact
//   sensors_position_of
// tracker: tracker_cue tracker_nearest_bearing_rate tracker_velocity
//   tracker_position tracker_last_seen tracker_configure
// nav: nav_zone nav_zone_margin nav_zone_contains nav_zone_distance_from
//   nav_storm nav_storm_margin nav_storm_contains nav_storm_distance_from
//   nav_hill nav_hill_margin nav_hill_contains nav_hill_distance_from
//   nav_pickups nav_nearest_pickup nav_field_distance
// score: score_me score_best score_armour
// angles: angles_normalize angles_angle_diff angles_bearing_to
//   angles_elevation_to angles_direction angles_distance angles_clamp
// status: status_text status_value status_get_text status_get_value
// raw: raw_number raw_text raw_has raw_flag raw_count raw_line raw_extra
// session: session_log
// Records: cy_contact, cy_vec3, cy_direction and the rest, in clunkyard.h.
// Angles are degrees from the +x axis; pitch is positive up.
// Manual: https://clunkyard.com/manual#wasm
#![no_std]
mod clunkyard;

use clunkyard::*;

/// Enough to turn against the brake in the force rulesets.
const SEARCH_TURN: f64 = 0.33;
/// How long to look again where a robot was last seen.
const MEMORY_TICKS: i32 = 60;

// A WebAssembly instance is one robot on one thread, so plain statics hold
// what it remembers between turns.
static mut FREE_FLIGHT: bool = false; // flying free, robots may be above or below
static mut LAST_SLOT: i32 = 0; // the robot seen most recently, 0 for none yet

// Each import writes its answer through a pointer and returns a status;
// these read one, or give None when it is absent.
fn get<T: Default>(read: unsafe extern "C" fn(*mut T) -> i32) -> Option<T> {
    let mut value = T::default();
    (unsafe { read(&mut value) } == CY_OK).then_some(value)
}
fn get_for<A, T: Default>(read: unsafe extern "C" fn(A, *mut T) -> i32, argument: A) -> Option<T> {
    let mut value = T::default();
    (unsafe { read(argument, &mut value) } == CY_OK).then_some(value)
}
fn angle_diff(target: f64, current: f64) -> f64 {
    let mut diff = 0.0;
    unsafe { cy_angles_angle_diff(target, current, &mut diff) };
    diff
}
fn bearing_to(from: &Vec3, to: &Vec3) -> f64 {
    let mut bearing = 0.0;
    unsafe { cy_angles_bearing_to(from, to, &mut bearing) };
    bearing
}
fn elevation_to(from: &Vec3, to: &Vec3) -> f64 {
    let mut elevation = 0.0;
    unsafe { cy_angles_elevation_to(from, to, &mut elevation) };
    elevation
}
fn unit(value: f64) -> f64 {
    value.clamp(-1.0, 1.0)
}

/// Drives towards a heading and an elevation, turning harder the smaller gain is.
fn steer(throttle: f64, heading: f64, elevation: f64, gain: f64, me_heading: f64, me_pitch: f64) {
    unsafe {
        cy_drive_throttle(throttle);
        cy_drive_turn(unit(angle_diff(heading, me_heading) / gain));
        cy_drive_pitch_turn(unit((elevation - me_pitch) / gain));
    }
}

fn start() {
    let is_3d = get(cy_match_is_3d).unwrap_or(0) == 1;
    let gravity = get(cy_arena_gravity).unwrap_or_default();
    unsafe { FREE_FLIGHT = is_3d && gravity.z == 0.0 };
}

fn turn() {
    let me = get(cy_me_position).unwrap_or_default();
    let heading = get(cy_me_heading).unwrap_or(0.0);
    let pitch = get(cy_me_pitch).unwrap_or(0.0);
    let tick = get(cy_clock_tick).unwrap_or(0);
    let (free_flight, last_slot) = unsafe { (FREE_FLIGHT, LAST_SLOT) };
    unsafe { cy_drive_brake(0.2) };

    let target = get(cy_sensors_nearest);
    // Where the robot seen last was, if that was in the last few seconds.
    let last = || {
        if last_slot == 0 {
            return None;
        }
        get_for(cy_tracker_last_seen, last_slot)
            .filter(|seen| tick - seen < MEMORY_TICKS)
            .and_then(|_| get_for(cy_tracker_position, last_slot))
    };
    if let Some(target) = target {
        unsafe { LAST_SLOT = target.slot };
        let throttle = if target.range > 350.0 { 0.7 } else if target.range < 200.0 { -0.4 } else { 0.2 };
        steer(throttle, target.bearing, target.elevation, 45.0, heading, pitch);
        unsafe {
            cy_scanner_heading(target.bearing); // narrow: holds it, reaching about 735
            cy_scanner_arc(20.0);
            cy_scanner_pitch(target.elevation);
        }
        // The heaviest shot that still covers the range in about a second,
        // aimed where the target will be, lobbed under gravity; straight at
        // it when no shot reaches it.
        let mut power = 1.0;
        unsafe { cy_gun_power_for_flight_time(target.range, 1.0, &mut power) };
        let mut aim = Solution::default();
        if unsafe { cy_gun_lead(&target, power, &mut aim) } != CY_OK {
            aim = Solution { heading: target.bearing, pitch: target.elevation, time: 0.0 };
        }
        let mut can_fire = 0;
        unsafe {
            cy_gun_aim_heading(aim.heading);
            cy_gun_aim_pitch(aim.pitch);
            cy_gun_can_fire(power, 10.0, &mut can_fire);
        }
        let now = get(cy_gun_aim_now).unwrap_or_default();
        let on_target = angle_diff(aim.heading, now.heading).abs() < 3.0 && (aim.pitch - now.pitch).abs() < 3.0;
        if on_target && can_fire == 1 {
            unsafe {
                cy_gun_fire();
                cy_gun_power(power);
            }
        }
    } else if let Some(last) = last() {
        // Head for where it was, sweeping a narrow scan across that bearing.
        let bearing = bearing_to(&me, &last);
        let elevation = if free_flight { elevation_to(&me, &last) } else { 0.0 };
        steer(0.7, bearing, elevation, 45.0, heading, pitch);
        unsafe {
            cy_scanner_heading(bearing + (tick % 5 - 2) as f64 * 8.0);
            cy_scanner_arc(30.0);
            cy_scanner_pitch(elevation);
        }
    } else if let Some(cue) = get(cy_tracker_cue) {
        // Head the way the cue points, sweeping the scan across its error.
        let bearing = bearing_to(&me, &cue.position);
        let elevation = if free_flight { elevation_to(&me, &cue.position) } else { 0.0 };
        let error = cue.bearing_error.min(30.0);
        steer(0.6, bearing, elevation, 45.0, heading, pitch);
        unsafe {
            cy_scanner_heading(bearing + (tick % 5 - 2) as f64 / 2.0 * error);
            cy_scanner_arc(30.0);
            cy_scanner_pitch(elevation);
        }
    } else {
        steer(0.5, heading, 0.0, 45.0, heading, pitch);
        // Sweeps the circle in half a second, flying free at three heights in turn.
        let scan = get(cy_scanner_last).unwrap_or_default();
        unsafe {
            cy_drive_turn(SEARCH_TURN);
            cy_scanner_heading(scan.heading + 36.0);
            cy_scanner_arc(40.0);
            cy_scanner_pitch(if free_flight { ((tick / 10) % 3 - 1) as f64 * 35.0 } else { 0.0 });
        }
    }

    // Low battery: drive gently so the shots can still be paid for.
    let charge = get(cy_battery_charge).unwrap_or(0.0);
    let capacity = get(cy_battery_capacity).unwrap_or(0.0);
    if charge < capacity / 4.0 {
        let throttle = get(cy_drive_get_throttle).unwrap_or(0.0);
        unsafe { cy_drive_throttle(throttle * 0.5) };
    }

    // The storm drifts across the arena and hurts anything inside it. Near its
    // edge, step out across its path: at right angles to its drift, on the
    // side this robot is already on, so it passes by.
    if let (Some(storm), Some(margin)) = (get(cy_nav_storm), get_for(cy_nav_storm_margin, &me as *const Vec3)) {
        if margin > -60.0 {
            let away = bearing_to(&storm.center, &me);
            let drift = bearing_to(&Vec3::default(), &storm.velocity);
            let side = if angle_diff(away, drift) < 0.0 { -90.0 } else { 90.0 };
            steer(1.0, drift + side, 0.0, 30.0, heading, pitch);
        }
    }

    // Sudden death: head back inside the safe zone.
    if let (Some(zone), Some(margin)) = (get(cy_nav_zone), get_for(cy_nav_zone_margin, &me as *const Vec3)) {
        if margin < 60.0 {
            let climb = if zone.shape == CY_SHAPE_SPHERE { elevation_to(&me, &zone.center) } else { 0.0 };
            steer(0.8, bearing_to(&me, &zone.center), climb, 30.0, heading, pitch);
        }
    }
}

cy_robot!(start, turn);
