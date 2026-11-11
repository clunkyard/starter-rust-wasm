//! Generated from protocol/v2/api.json by protocol/v2/codegen/wasm-surfaces.js; do not edit. Run `make api`.
//!
//! Clunkyard WebAssembly ABI 4 for Rust: every clunkyard.* import at its
//! C name (cy_sensors_nearest_v1, with cy_sensors_nearest for the latest
//! version), the records and the constants. Each import returns a status
//! (CY_OK, CY_NONE for absent) and writes through pointers; each costs fuel.
//! The crate is #![no_std]; this module brings the panic handler, which
//! traps. cy_robot! writes the module's two exports. Build with
//! rustc --target wasm32-unknown-unknown --crate-type cdylib -O.
//!
//! ```ignore
//! #![no_std]
//! mod clunkyard;
//! use clunkyard::*;
//! fn turn() {
//!     let mut target = Contact::default();
//!     if unsafe { cy_sensors_nearest(&mut target) } == CY_OK {
//!         unsafe { cy_gun_aim_heading(target.bearing); cy_gun_fire(); }
//!     }
//! }
//! cy_robot!(|| {}, turn);
//! ```
#![allow(dead_code, non_camel_case_types, non_upper_case_globals, unused_imports, clippy::all)]

pub const CY_API_VERSION: &str = "2.0.0";
pub const CY_API_MAJOR: i32 = 2;
pub const CY_API_MINOR: i32 = 0;
/// Done.
pub const CY_OK: i32 = 0;
/// Absent, which is not an error: no precise contact, a mechanic that is off, a shot that cannot reach, or a field that is unset.
pub const CY_NONE: i32 = 1;
/// The buffer is too short; the length or count says how much is needed.
pub const CY_SHORT: i32 = 2;
/// A number is NaN or infinite; the field is left unset.
pub const CY_E_NOT_FINITE: i32 = -1;
/// A number is outside what the call accepts.
pub const CY_E_RANGE: i32 = -2;
/// An argument is malformed: a null pointer, a slot out of range, text that is not UTF-8.
pub const CY_E_ARGUMENT: i32 = -3;
/// The wrong point in the session, such as a state query before the first state.
pub const CY_E_PHASE: i32 = -4;
/// The binding's API version is newer than the core's, or a different major.
pub const CY_E_VERSION: i32 = -5;
/// A fault in the core. It is never the robot's fault; the handle resets on the next state.
pub const CY_E_INTERNAL: i32 = -6;
/// source: none
pub const CY_SOURCE_NONE: i32 = 0;
/// source: scan
pub const CY_SOURCE_SCAN: i32 = 1;
/// source: proximity
pub const CY_SOURCE_PROXIMITY: i32 = 2;
/// source: faint
pub const CY_SOURCE_FAINT: i32 = 3;
/// source: beam
pub const CY_SOURCE_BEAM: i32 = 4;
/// source: muzzle
pub const CY_SOURCE_MUZZLE: i32 = 5;
/// source: motion
pub const CY_SOURCE_MOTION: i32 = 6;
/// source: ping
pub const CY_SOURCE_PING: i32 = 7;
/// mode: none
pub const CY_MODE_NONE: i32 = 0;
/// mode: 2d
pub const CY_MODE_TWO_D: i32 = 1;
/// mode: 3d
pub const CY_MODE_THREE_D: i32 = 2;
/// ruleset: none
pub const CY_RULESET_NONE: i32 = 0;
/// ruleset: classic
pub const CY_RULESET_CLASSIC: i32 = 1;
/// ruleset: inertial-2d
pub const CY_RULESET_INERTIAL_2D: i32 = 2;
/// ruleset: gravity-flat
pub const CY_RULESET_GRAVITY_FLAT: i32 = 3;
/// ruleset: gravity-hills
pub const CY_RULESET_GRAVITY_HILLS: i32 = 4;
/// ruleset: zero-g
pub const CY_RULESET_ZERO_G: i32 = 5;
/// control: none
pub const CY_CONTROL_NONE: i32 = 0;
/// control: direct
pub const CY_CONTROL_DIRECT: i32 = 1;
/// control: forces
pub const CY_CONTROL_FORCES: i32 = 2;
/// terrain_kind: none
pub const CY_TERRAIN_KIND_NONE: i32 = 1;
/// terrain_kind: flat
pub const CY_TERRAIN_KIND_FLAT: i32 = 2;
/// terrain_kind: hills
pub const CY_TERRAIN_KIND_HILLS: i32 = 3;
/// shape: none
pub const CY_SHAPE_NONE: i32 = 0;
/// shape: circle
pub const CY_SHAPE_CIRCLE: i32 = 1;
/// shape: sphere
pub const CY_SHAPE_SPHERE: i32 = 2;
/// shape: column
pub const CY_SHAPE_COLUMN: i32 = 3;
/// pickup_kind: any
pub const CY_PICKUP_KIND_ANY: i32 = 0;
/// pickup_kind: energy
pub const CY_PICKUP_KIND_ENERGY: i32 = 1;
/// pickup_kind: health
pub const CY_PICKUP_KIND_HEALTH: i32 = 2;
/// pickup_kind: armour
pub const CY_PICKUP_KIND_ARMOUR: i32 = 3;
/// compute_unit: none
pub const CY_COMPUTE_UNIT_NONE: i32 = 0;
/// compute_unit: operations
pub const CY_COMPUTE_UNIT_OPERATIONS: i32 = 1;
/// compute_unit: fuel
pub const CY_COMPUTE_UNIT_FUEL: i32 = 2;
/// compute_unit: cpu-ms
pub const CY_COMPUTE_UNIT_CPU_MS: i32 = 3;
/// event: none
pub const CY_EVENT_NONE: i32 = 0;
/// event: hello
pub const CY_EVENT_HELLO: i32 = 1;
/// event: state
pub const CY_EVENT_STATE: i32 = 2;
/// event: end
pub const CY_EVENT_END: i32 = 3;

/// A vector in world axes. 24 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// A rotation rate or torque about the robot's own axes. 24 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rotation {
    pub yaw: f64,
    pub pitch: f64,
    pub roll: f64,
}

/// A direction as a heading and an elevation. 16 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Direction {
    pub heading: f64,
    pub pitch: f64,
}

/// The chassis axes as unit vectors in world axes. 72 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AttitudeV1 {
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}
pub type Attitude = AttitudeV1;

/// What the previous tick applied to you, after the battery scaled the drive. 56 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AppliedV1 {
    pub force: Vec3,
    pub torque: Rotation,
    pub brake: f64,
}
pub type Applied = AppliedV1;

/// The scan that produced this tick's contacts: the one your previous act asked for. 32 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScanV1 {
    pub heading: f64,
    pub pitch: f64,
    pub arc: f64,
    pub range: f64,
}
pub type Scan = ScanV1;

/// A robot the sensors detect this tick. 72 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ContactV1 {
    pub slot: i32,
    pub source: i32,
    pub range: f64,
    pub bearing: f64,
    pub elevation: f64,
    pub bearing_error: f64,
    pub range_error: f64,
    pub position: Vec3,
}
pub type Contact = ContactV1;

/// The most accurate passive contact the Tracker holds: where to look, not where to shoot. 40 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CueV1 {
    pub source: i32,
    pub tick: i32,
    pub bearing_error: f64,
    pub position: Vec3,
}
pub type Cue = CueV1;

/// Where to aim for a shot fired now to meet a target, and when it meets it. 24 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SolutionV1 {
    pub heading: f64,
    pub pitch: f64,
    pub time: f64,
}
pub type Solution = SolutionV1;

/// The sudden-death safe zone. 40 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ZoneV1 {
    pub shape: i32,
    pub reserved: i32,
    pub radius: f64,
    pub center: Vec3,
}
pub type Zone = ZoneV1;

/// The storm now. 72 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StormV1 {
    pub shape: i32,
    pub reserved: i32,
    pub radius: f64,
    pub dps: f64,
    pub center: Vec3,
    pub velocity: Vec3,
}
pub type Storm = StormV1;

/// The king-of-the-hill zone now. 80 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HillV1 {
    pub shape: i32,
    pub holder: i32,
    pub contested: i32,
    pub moves_tick: i32,
    pub held_ticks: i32,
    pub best_held_ticks: i32,
    pub radius: f64,
    pub center: Vec3,
    pub next: Vec3,
}
pub type Hill = HillV1;

/// A cell on the field. 40 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PickupV1 {
    pub kind: i32,
    pub expires_tick: i32,
    pub amount: f64,
    pub position: Vec3,
}
pub type Pickup = PickupV1;

/// How the match charges compute against the battery. 16 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ComputeV1 {
    pub k: f64,
    pub quanta: i32,
    pub unit: i32,
}
pub type Compute = ComputeV1;

#[link(wasm_import_module = "clunkyard")]
unsafe extern "C" {
    /// The match is 2D or 3D.
    #[link_name = "match_mode"]
    pub fn cy_match_mode(out: *mut i32) -> i32;
    /// The ruleset in play.
    #[link_name = "match_ruleset"]
    pub fn cy_match_ruleset(out: *mut i32) -> i32;
    /// Whether the drive sets speeds (direct) or forces.
    #[link_name = "match_control"]
    pub fn cy_match_control(out: *mut i32) -> i32;
    /// Robots in the match, you included; slots run 1 to this.
    #[link_name = "match_robots"]
    pub fn cy_match_robots(out: *mut i32) -> i32;
    /// The match is 3D.
    #[link_name = "match_is_3d"]
    pub fn cy_match_is_3d(out: *mut i32) -> i32;
    /// Ticks per second.
    #[link_name = "match_tick_rate"]
    pub fn cy_match_tick_rate(out: *mut i32) -> i32;
    /// The last tick the match can reach.
    #[link_name = "match_max_ticks"]
    pub fn cy_match_max_ticks(out: *mut i32) -> i32;
    /// The id the protocol gives the robot in a slot.
    #[link_name = "robot_id"]
    pub fn cy_robot_id(slot: i32, buf: *mut u8, capacity: u32, length: *mut u32) -> i32;
    /// This tick, from 0.
    #[link_name = "clock_tick"]
    pub fn cy_clock_tick(out: *mut i32) -> i32;
    /// The match time in seconds.
    #[link_name = "clock_seconds"]
    pub fn cy_clock_seconds(out: *mut f64) -> i32;
    /// Ticks left: match.max_ticks less this tick.
    #[link_name = "clock_remaining_ticks"]
    pub fn cy_clock_remaining_ticks(out: *mut i32) -> i32;
    /// The arena runs from 0 to this on each axis.
    #[link_name = "arena_size"]
    pub fn cy_arena_size(out: *mut Vec3) -> i32;
    /// The acceleration of gravity, units/s².
    #[link_name = "arena_gravity"]
    pub fn cy_arena_gravity(out: *mut Vec3) -> i32;
    /// The kind of ground: none, flat or hills.
    #[link_name = "arena_terrain_kind"]
    pub fn cy_arena_terrain_kind(out: *mut i32) -> i32;
    /// The height of the ground at a point, interpolated as the arena does.
    #[link_name = "arena_ground_height"]
    pub fn cy_arena_ground_height(x: f64, y: f64, out: *mut f64) -> i32;
    /// Health left.
    #[link_name = "me_health"]
    pub fn cy_me_health(out: *mut f64) -> i32;
    /// Your centre.
    #[link_name = "me_position"]
    pub fn cy_me_position(out: *mut Vec3) -> i32;
    /// Your velocity, units/s.
    #[link_name = "me_velocity"]
    pub fn cy_me_velocity(out: *mut Vec3) -> i32;
    /// Your chassis heading.
    #[link_name = "me_heading"]
    pub fn cy_me_heading(out: *mut f64) -> i32;
    /// Your chassis pitch; 0 in 2D.
    #[link_name = "me_pitch"]
    pub fn cy_me_pitch(out: *mut f64) -> i32;
    /// Your chassis roll; 0 in 2D and classic.
    #[link_name = "me_roll"]
    pub fn cy_me_roll(out: *mut f64) -> i32;
    /// Your rotation rates, degrees/s.
    #[link_name = "me_angular_velocity"]
    pub fn cy_me_angular_velocity(out: *mut Rotation) -> i32;
    /// Your chassis axes in world axes.
    #[link_name = "me_attitude_v1"]
    pub fn cy_me_attitude_v1(out: *mut AttitudeV1) -> i32;
    /// You rest on the ground, under gravity.
    #[link_name = "me_grounded"]
    pub fn cy_me_grounded(out: *mut i32) -> i32;
    /// In 3D, the height of your underside above the ground.
    #[link_name = "me_altitude"]
    pub fn cy_me_altitude(out: *mut f64) -> i32;
    /// What the previous tick applied to you.
    #[link_name = "me_applied_v1"]
    pub fn cy_me_applied_v1(out: *mut AppliedV1) -> i32;
    /// Charge now, before this tick's regeneration and driving.
    #[link_name = "battery_charge"]
    pub fn cy_battery_charge(out: *mut f64) -> i32;
    /// Charge when full.
    #[link_name = "battery_capacity"]
    pub fn cy_battery_capacity(out: *mut f64) -> i32;
    /// Charge regained each second.
    #[link_name = "battery_regen"]
    pub fn cy_battery_regen(out: *mut f64) -> i32;
    /// The drive fraction regeneration pays for forever.
    #[link_name = "battery_sustainable_drive"]
    pub fn cy_battery_sustainable_drive(out: *mut f64) -> i32;
    /// How compute is charged, when the match charges it.
    #[link_name = "battery_compute_v1"]
    pub fn cy_battery_compute_v1(out: *mut ComputeV1) -> i32;
    /// What holding a drive fraction costs each second, by hello.formulas.driveCost.
    #[link_name = "battery_drive_cost"]
    pub fn cy_battery_drive_cost(fraction: f64, out: *mut f64) -> i32;
    /// The forces.lift that cancels gravity for a level robot; 0 without gravity.
    #[link_name = "drive_hover_lift"]
    pub fn cy_drive_hover_lift(out: *mut f64) -> i32;
    /// Ask for direct controls this tick; the arena decides what they do in the ruleset.
    #[link_name = "drive_throttle"]
    pub fn cy_drive_throttle(value: f64) -> i32;
    /// Ask for direct controls this tick; the arena decides what they do in the ruleset.
    #[link_name = "drive_turn"]
    pub fn cy_drive_turn(value: f64) -> i32;
    /// Ask for direct controls this tick; the arena decides what they do in the ruleset.
    #[link_name = "drive_pitch_turn"]
    pub fn cy_drive_pitch_turn(value: f64) -> i32;
    /// Ask for forces and torques this tick, in the force rulesets.
    #[link_name = "drive_thrust"]
    pub fn cy_drive_thrust(value: f64) -> i32;
    /// Ask for forces and torques this tick, in the force rulesets.
    #[link_name = "drive_strafe"]
    pub fn cy_drive_strafe(value: f64) -> i32;
    /// Ask for forces and torques this tick, in the force rulesets.
    #[link_name = "drive_lift"]
    pub fn cy_drive_lift(value: f64) -> i32;
    /// Ask for forces and torques this tick, in the force rulesets.
    #[link_name = "drive_yaw_torque"]
    pub fn cy_drive_yaw_torque(value: f64) -> i32;
    /// Ask for forces and torques this tick, in the force rulesets.
    #[link_name = "drive_pitch_torque"]
    pub fn cy_drive_pitch_torque(value: f64) -> i32;
    /// Ask for forces and torques this tick, in the force rulesets.
    #[link_name = "drive_roll_torque"]
    pub fn cy_drive_roll_torque(value: f64) -> i32;
    /// Ask for forces and torques this tick, in the force rulesets.
    #[link_name = "drive_brake"]
    pub fn cy_drive_brake(value: f64) -> i32;
    /// What this tick's act holds for a drive field so far, as set.
    #[link_name = "drive_get_throttle"]
    pub fn cy_drive_get_throttle(out: *mut f64) -> i32;
    /// What this tick's act holds for a drive field so far, as set.
    #[link_name = "drive_get_turn"]
    pub fn cy_drive_get_turn(out: *mut f64) -> i32;
    /// What this tick's act holds for a drive field so far, as set.
    #[link_name = "drive_get_pitch_turn"]
    pub fn cy_drive_get_pitch_turn(out: *mut f64) -> i32;
    /// What this tick's act holds for a drive field so far, as set.
    #[link_name = "drive_get_thrust"]
    pub fn cy_drive_get_thrust(out: *mut f64) -> i32;
    /// What this tick's act holds for a drive field so far, as set.
    #[link_name = "drive_get_strafe"]
    pub fn cy_drive_get_strafe(out: *mut f64) -> i32;
    /// What this tick's act holds for a drive field so far, as set.
    #[link_name = "drive_get_lift"]
    pub fn cy_drive_get_lift(out: *mut f64) -> i32;
    /// What this tick's act holds for a drive field so far, as set.
    #[link_name = "drive_get_yaw_torque"]
    pub fn cy_drive_get_yaw_torque(out: *mut f64) -> i32;
    /// What this tick's act holds for a drive field so far, as set.
    #[link_name = "drive_get_pitch_torque"]
    pub fn cy_drive_get_pitch_torque(out: *mut f64) -> i32;
    /// What this tick's act holds for a drive field so far, as set.
    #[link_name = "drive_get_roll_torque"]
    pub fn cy_drive_get_roll_torque(out: *mut f64) -> i32;
    /// What this tick's act holds for a drive field so far, as set.
    #[link_name = "drive_get_brake"]
    pub fn cy_drive_get_brake(out: *mut f64) -> i32;
    /// Where the turret points now; a shot leaves along it.
    #[link_name = "gun_aim_now"]
    pub fn cy_gun_aim_now(out: *mut Direction) -> i32;
    /// Ticks until the gun can fire; 0 means it can fire this tick.
    #[link_name = "gun_cooldown_ticks"]
    pub fn cy_gun_cooldown_ticks(out: *mut i32) -> i32;
    /// The aim spread a shot would have at your speed now.
    #[link_name = "gun_spread"]
    pub fn cy_gun_spread(out: *mut f64) -> i32;
    /// The heaviest shot the battery pays for now; 0 when it cannot pay for the lightest.
    #[link_name = "gun_max_power"]
    pub fn cy_gun_max_power(out: *mut f64) -> i32;
    /// The escalation multiplier on hit damage this tick.
    #[link_name = "gun_damage_scale"]
    pub fn cy_gun_damage_scale(out: *mut f64) -> i32;
    /// The cooldown is 0 and the battery pays for a shot at this power and still holds the reserve.
    #[link_name = "gun_can_fire"]
    pub fn cy_gun_can_fire(power: f64, reserve: f64, out: *mut i32) -> i32;
    /// How fast a shot at this power flies, units/s, by hello.formulas.shotSpeed.
    #[link_name = "gun_shot_speed"]
    pub fn cy_gun_shot_speed(power: f64, out: *mut f64) -> i32;
    /// What a shot at this power costs the battery, by hello.formulas.shotCost.
    #[link_name = "gun_shot_cost"]
    pub fn cy_gun_shot_cost(power: f64, out: *mut f64) -> i32;
    /// The reload after a shot at this power, ticks, by hello.formulas.reloadTicks.
    #[link_name = "gun_reload_ticks"]
    pub fn cy_gun_reload_ticks(power: f64, out: *mut f64) -> i32;
    /// The damage of a hit at this power before escalation, by hello.formulas.shotDamage.
    #[link_name = "gun_shot_damage"]
    pub fn cy_gun_shot_damage(power: f64, out: *mut f64) -> i32;
    /// The power whose shot flies at this speed, within the legal powers, by hello.formulas.powerForSpeed.
    #[link_name = "gun_power_for_speed"]
    pub fn cy_gun_power_for_speed(speed: f64, out: *mut f64) -> i32;
    /// The power whose shot covers this range in this time, within the legal powers.
    #[link_name = "gun_power_for_flight_time"]
    pub fn cy_gun_power_for_flight_time(range: f64, seconds: f64, out: *mut f64) -> i32;
    /// The aim at which a shot fired now meets the contact if it holds its tracked velocity; absent when no shot reaches it.
    #[link_name = "gun_lead_v1"]
    pub fn cy_gun_lead_v1(contact: *const ContactV1, power: f64, out: *mut SolutionV1) -> i32;
    /// The aim at which a shot fired now meets a point moving at this velocity; absent when no shot reaches it.
    #[link_name = "gun_lead_point_v1"]
    pub fn cy_gun_lead_point_v1(position: *const Vec3, velocity: *const Vec3, power: f64, out: *mut SolutionV1) -> i32;
    /// Ask the turret to turn toward a direction; it slews at gun.turnRate.
    #[link_name = "gun_aim_heading"]
    pub fn cy_gun_aim_heading(value: f64) -> i32;
    /// Ask the turret to turn toward a direction; it slews at gun.turnRate.
    #[link_name = "gun_aim_pitch"]
    pub fn cy_gun_aim_pitch(value: f64) -> i32;
    /// Ask to fire this tick; the arena decides whether the gun and battery allow it.
    #[link_name = "gun_fire"]
    pub fn cy_gun_fire() -> i32;
    /// Ask to fire this tick; the arena decides whether the gun and battery allow it.
    #[link_name = "gun_power"]
    pub fn cy_gun_power(value: f64) -> i32;
    /// What this tick's act holds for a gun field so far, as set.
    #[link_name = "gun_get_aim_heading"]
    pub fn cy_gun_get_aim_heading(out: *mut f64) -> i32;
    /// What this tick's act holds for a gun field so far, as set.
    #[link_name = "gun_get_aim_pitch"]
    pub fn cy_gun_get_aim_pitch(out: *mut f64) -> i32;
    /// What this tick's act holds for a gun field so far, as set.
    #[link_name = "gun_get_fire"]
    pub fn cy_gun_get_fire(out: *mut f64) -> i32;
    /// What this tick's act holds for a gun field so far, as set.
    #[link_name = "gun_get_power"]
    pub fn cy_gun_get_power(out: *mut f64) -> i32;
    /// The scan that produced this tick's contacts.
    #[link_name = "scanner_last_v1"]
    pub fn cy_scanner_last_v1(out: *mut ScanV1) -> i32;
    /// Robots this close are detected in any direction.
    #[link_name = "scanner_proximity_range"]
    pub fn cy_scanner_proximity_range(out: *mut f64) -> i32;
    /// How far a scan of this arc reaches, by hello.formulas.reach.
    #[link_name = "scanner_reach"]
    pub fn cy_scanner_reach(arc: f64, out: *mut f64) -> i32;
    /// The widest arc that still reaches this far, by hello.formulas.arcForReach.
    #[link_name = "scanner_arc_for_reach"]
    pub fn cy_scanner_arc_for_reach(range: f64, out: *mut f64) -> i32;
    /// Ask where the scanner points for the next state.
    #[link_name = "scanner_heading"]
    pub fn cy_scanner_heading(value: f64) -> i32;
    /// Ask where the scanner points for the next state.
    #[link_name = "scanner_arc"]
    pub fn cy_scanner_arc(value: f64) -> i32;
    /// Ask where the scanner points for the next state.
    #[link_name = "scanner_pitch"]
    pub fn cy_scanner_pitch(value: f64) -> i32;
    /// What this tick's act holds for a scanner field so far, as set.
    #[link_name = "scanner_get_heading"]
    pub fn cy_scanner_get_heading(out: *mut f64) -> i32;
    /// What this tick's act holds for a scanner field so far, as set.
    #[link_name = "scanner_get_arc"]
    pub fn cy_scanner_get_arc(out: *mut f64) -> i32;
    /// What this tick's act holds for a scanner field so far, as set.
    #[link_name = "scanner_get_pitch"]
    pub fn cy_scanner_get_pitch(out: *mut f64) -> i32;
    /// Every contact this tick: scan and proximity first, nearest first, then passive ones.
    #[link_name = "sensors_contacts_v1"]
    pub fn cy_sensors_contacts_v1(out: *mut ContactV1, capacity: u32, count: *mut u32) -> i32;
    /// The nearest precise contact this tick. Passive contacts are never returned.
    #[link_name = "sensors_nearest_v1"]
    pub fn cy_sensors_nearest_v1(out: *mut ContactV1) -> i32;
    /// This tick's passive contacts, most accurate (smallest bearing error) first.
    #[link_name = "sensors_cues_v1"]
    pub fn cy_sensors_cues_v1(out: *mut ContactV1, capacity: u32, count: *mut u32) -> i32;
    /// The precise contact for a robot this tick, if it was detected.
    #[link_name = "sensors_contact_v1"]
    pub fn cy_sensors_contact_v1(slot: i32, out: *mut ContactV1) -> i32;
    /// Where a contact places its robot: your position plus its range along its bearing and elevation.
    #[link_name = "sensors_position_of_v1"]
    pub fn cy_sensors_position_of_v1(contact: *const ContactV1, out: *mut Vec3) -> i32;
    /// The most accurate passive contact heard in the last cue_ticks (60 by default), or absent.
    #[link_name = "tracker_cue_v1"]
    pub fn cy_tracker_cue_v1(out: *mut CueV1) -> i32;
    /// How fast the nearest contact's bearing turns, from two sightings.
    #[link_name = "tracker_nearest_bearing_rate"]
    pub fn cy_tracker_nearest_bearing_rate(out: *mut f64) -> i32;
    /// The robot's tracked velocity; zero when it has no track.
    #[link_name = "tracker_velocity"]
    pub fn cy_tracker_velocity(slot: i32, out: *mut Vec3) -> i32;
    /// Where the robot was last seen, if it was.
    #[link_name = "tracker_position"]
    pub fn cy_tracker_position(slot: i32, out: *mut Vec3) -> i32;
    /// The tick the robot was last seen, if it was.
    #[link_name = "tracker_last_seen"]
    pub fn cy_tracker_last_seen(slot: i32, out: *mut i32) -> i32;
    /// Sets the Tracker's parameters, in the robot's start only; after the first state it refuses with E_PHASE.
    #[link_name = "tracker_configure"]
    pub fn cy_tracker_configure(smoothing: f64, forget_ticks: i32, cue_ticks: i32) -> i32;
    /// The sudden-death safe zone, once sudden death starts.
    #[link_name = "nav_zone_v1"]
    pub fn cy_nav_zone_v1(out: *mut ZoneV1) -> i32;
    /// How far inside the zone a point is, by its shape's distance; absent while there is none.
    #[link_name = "nav_zone_margin"]
    pub fn cy_nav_zone_margin(point: *const Vec3, out: *mut f64) -> i32;
    /// The point is inside the zone by at least the margin; absent while there is none.
    #[link_name = "nav_zone_contains"]
    pub fn cy_nav_zone_contains(point: *const Vec3, margin: f64, out: *mut i32) -> i32;
    /// How far a point is from the zone's edge, 0 inside; absent while there is none.
    #[link_name = "nav_zone_distance_from"]
    pub fn cy_nav_zone_distance_from(point: *const Vec3, out: *mut f64) -> i32;
    /// The storm, once it enters.
    #[link_name = "nav_storm_v1"]
    pub fn cy_nav_storm_v1(out: *mut StormV1) -> i32;
    /// How far inside the storm a point is, by its shape's distance; absent while there is none.
    #[link_name = "nav_storm_margin"]
    pub fn cy_nav_storm_margin(point: *const Vec3, out: *mut f64) -> i32;
    /// The point is inside the storm by at least the margin; absent while there is none.
    #[link_name = "nav_storm_contains"]
    pub fn cy_nav_storm_contains(point: *const Vec3, margin: f64, out: *mut i32) -> i32;
    /// How far a point is from the storm's edge, 0 inside; absent while there is none.
    #[link_name = "nav_storm_distance_from"]
    pub fn cy_nav_storm_distance_from(point: *const Vec3, out: *mut f64) -> i32;
    /// The hill, once it appears.
    #[link_name = "nav_hill_v1"]
    pub fn cy_nav_hill_v1(out: *mut HillV1) -> i32;
    /// How far inside the hill a point is, by its shape's distance; absent while there is none.
    #[link_name = "nav_hill_margin"]
    pub fn cy_nav_hill_margin(point: *const Vec3, out: *mut f64) -> i32;
    /// The point is inside the hill by at least the margin; absent while there is none.
    #[link_name = "nav_hill_contains"]
    pub fn cy_nav_hill_contains(point: *const Vec3, margin: f64, out: *mut i32) -> i32;
    /// How far a point is from the hill's edge, 0 inside; absent while there is none.
    #[link_name = "nav_hill_distance_from"]
    pub fn cy_nav_hill_distance_from(point: *const Vec3, out: *mut f64) -> i32;
    /// Every pickup on the field.
    #[link_name = "nav_pickups_v1"]
    pub fn cy_nav_pickups_v1(out: *mut PickupV1, capacity: u32, count: *mut u32) -> i32;
    /// The pickup of this kind nearest you by field distance, if there is one.
    #[link_name = "nav_nearest_pickup_v1"]
    pub fn cy_nav_nearest_pickup_v1(kind: i32, out: *mut PickupV1) -> i32;
    /// The distance between two points as the field measures it: in the plane under gravity, in space otherwise.
    #[link_name = "nav_field_distance"]
    pub fn cy_nav_field_distance(a: *const Vec3, b: *const Vec3, out: *mut f64) -> i32;
    /// Your score; absent when the match does not score.
    #[link_name = "score_me"]
    pub fn cy_score_me(out: *mut f64) -> i32;
    /// The best other robot's score; absent when the match does not score.
    #[link_name = "score_best"]
    pub fn cy_score_best(out: *mut f64) -> i32;
    /// Your armour; absent when the match has no objectives.
    #[link_name = "score_armour"]
    pub fn cy_score_armour(out: *mut f64) -> i32;
    /// An angle brought into 0 to 360, exactly: degrees % 360, plus 360 when that is negative.
    #[link_name = "angles_normalize"]
    pub fn cy_angles_normalize(degrees: f64, out: *mut f64) -> i32;
    /// The signed turn from current to target: normalize(target - current + 540) - 180.
    #[link_name = "angles_angle_diff"]
    pub fn cy_angles_angle_diff(target: f64, current: f64, out: *mut f64) -> i32;
    /// The heading from one point to another.
    #[link_name = "angles_bearing_to"]
    pub fn cy_angles_bearing_to(from: *const Vec3, to: *const Vec3, out: *mut f64) -> i32;
    /// The elevation from one point to another.
    #[link_name = "angles_elevation_to"]
    pub fn cy_angles_elevation_to(from: *const Vec3, to: *const Vec3, out: *mut f64) -> i32;
    /// The unit vector along a heading and pitch.
    #[link_name = "angles_direction"]
    pub fn cy_angles_direction(heading: f64, pitch: f64, out: *mut Vec3) -> i32;
    /// The straight-line distance between two points.
    #[link_name = "angles_distance"]
    pub fn cy_angles_distance(a: *const Vec3, b: *const Vec3, out: *mut f64) -> i32;
    /// A value held within low and high.
    #[link_name = "angles_clamp"]
    pub fn cy_angles_clamp(value: f64, low: f64, high: f64, out: *mut f64) -> i32;
    /// Set the status line; leaving it out keeps the last one.
    #[link_name = "status_text"]
    pub fn cy_status_text(text: *const u8, text_length: u32) -> i32;
    /// Set the status line; leaving it out keeps the last one.
    #[link_name = "status_value"]
    pub fn cy_status_value(value: f64) -> i32;
    /// What this tick's act holds for the status line so far, as set.
    #[link_name = "status_get_text"]
    pub fn cy_status_get_text(buf: *mut u8, capacity: u32, length: *mut u32) -> i32;
    /// What this tick's act holds for the status line so far, as set.
    #[link_name = "status_get_value"]
    pub fn cy_status_get_value(out: *mut f64) -> i32;
    /// The number at a path, if there is one.
    #[link_name = "raw_number"]
    pub fn cy_raw_number(path: *const u8, path_length: u32, out: *mut f64) -> i32;
    /// The string at a path, if there is one.
    #[link_name = "raw_text"]
    pub fn cy_raw_text(path: *const u8, path_length: u32, buf: *mut u8, capacity: u32, length: *mut u32) -> i32;
    /// Something is at the path.
    #[link_name = "raw_has"]
    pub fn cy_raw_has(path: *const u8, path_length: u32, out: *mut i32) -> i32;
    /// The boolean at a path, if there is one.
    #[link_name = "raw_flag"]
    pub fn cy_raw_flag(path: *const u8, path_length: u32, out: *mut i32) -> i32;
    /// The length of the array at a path, if there is one.
    #[link_name = "raw_count"]
    pub fn cy_raw_count(path: *const u8, path_length: u32, out: *mut i32) -> i32;
    /// The last line the session read: the hello, or the latest state.
    #[link_name = "raw_line"]
    pub fn cy_raw_line(buf: *mut u8, capacity: u32, length: *mut u32) -> i32;
    /// Adds fields the API does not cover to this tick's act; a key the API writes (v, type, tick, drive, forces, aim, scan, fire, status) is refused with E_ARGUMENT.
    #[link_name = "raw_extra"]
    pub fn cy_raw_extra(json: *const u8, json_length: u32) -> i32;
    /// Writes a line to standard error, never to the protocol.
    #[link_name = "session_log"]
    pub fn cy_session_log(line: *const u8, line_length: u32) -> i32;
    /// The seed for the robot's own randomness.
    #[link_name = "robot_seed"]
    pub fn cy_robot_seed(out: *mut u64) -> i32;
}

pub use self::cy_me_attitude_v1 as cy_me_attitude;
pub use self::cy_me_applied_v1 as cy_me_applied;
pub use self::cy_battery_compute_v1 as cy_battery_compute;
pub use self::cy_gun_lead_v1 as cy_gun_lead;
pub use self::cy_gun_lead_point_v1 as cy_gun_lead_point;
pub use self::cy_scanner_last_v1 as cy_scanner_last;
pub use self::cy_sensors_contacts_v1 as cy_sensors_contacts;
pub use self::cy_sensors_nearest_v1 as cy_sensors_nearest;
pub use self::cy_sensors_cues_v1 as cy_sensors_cues;
pub use self::cy_sensors_contact_v1 as cy_sensors_contact;
pub use self::cy_sensors_position_of_v1 as cy_sensors_position_of;
pub use self::cy_tracker_cue_v1 as cy_tracker_cue;
pub use self::cy_nav_zone_v1 as cy_nav_zone;
pub use self::cy_nav_storm_v1 as cy_nav_storm;
pub use self::cy_nav_hill_v1 as cy_nav_hill;
pub use self::cy_nav_pickups_v1 as cy_nav_pickups;
pub use self::cy_nav_nearest_pickup_v1 as cy_nav_nearest_pickup;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

/// The module's exports: start runs once after the hello, turn once a state.
#[macro_export]
macro_rules! cy_robot {
    ($start:expr, $turn:expr) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn clunkyard_start() -> i32 {
            ($start)();
            0
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn clunkyard_turn() -> i32 {
            ($turn)();
            0
        }
    };
}
