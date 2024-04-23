//! ds implementation for trait Tdrawstuff and setter to change drawstuff later
//!

use crate::ode::ds::{dsFunctions_C, Tdrawstuff};

use std::ffi::{c_uint, c_int, c_char};

pub use drawstuff::drawstuff::*;

/// Drawstuff
pub struct Drawstuff {
}

/// Tdrawstuff for Drawstuff
impl Tdrawstuff for Drawstuff {
  /// dsDebug void dsDebug(const char *msg, ...);
  fn Debug(&self, msg: *const c_char) {
unsafe {
    dsDebug(msg); // and some varargs
}
    // print!("{}", String::from_u8(unsafe { std::slice::from_raw_parts(msg as *const u8, c strlen(msg)) }));
  }
  /// dsError void dsError(const char *msg, ...);
  fn Error(&self, msg: *const c_char) {
unsafe {
    dsError(msg); // and some varargs
}
    // print!("{}", String::from_u8(unsafe { std::slice::from_raw_parts(msg as *const u8, c strlen(msg)) }));
  }
  /// dsPrint void dsPrint(const char *msg, ...);
  fn Print(&self, msg: *const c_char) {
unsafe {
    dsPrint(msg); // and some varargs
}
    // print!("{}", String::from_u8(unsafe { std::slice::from_raw_parts(msg as *const u8, c strlen(msg)) }));
  }

  /// dsDrawBox 3 12 3
  fn DrawBox(&self, pos: *const f32, rot: *const f32, lxyz: *const f32) {
unsafe {
    dsDrawBox(pos, rot, lxyz);
}
  }
  /// dsDrawBoxD 3 12 3
  fn DrawBoxD(&self, pos: *const f64, rot: *const f64, lxyz: *const f64) {
unsafe {
    dsDrawBoxD(pos, rot, lxyz);
}
  }
  /// dsDrawCapsule 3 12
  fn DrawCapsule(&self, pos: *const f32, rot: *const f32, l: f32, r: f32) {
unsafe {
    dsDrawCapsule(pos, rot, l, r);
}
  }
  /// dsDrawCapsuleD 3 12
  fn DrawCapsuleD(&self, pos: *const f64, rot: *const f64, l: f32, r: f32) {
unsafe {
    dsDrawCapsuleD(pos, rot, l, r);
}
  }
  /// dsDrawConvex 3 12 planecount pointcount x
  fn DrawConvex(&self, pos: *const f32, rot: *const f32,
    planes: *const f32, planecount: c_uint,
    points: *const f32, pointcount: c_uint,
    polygons: *const c_uint) {
unsafe {
    dsDrawConvex(pos, rot, planes, planecount, points, pointcount, polygons);
}
  }
  /// dsDrawConvexD 3 12 planecount pointcount x
  fn DrawConvexD(&self, pos: *const f64, rot: *const f64,
    planes: *const f64, planecount: c_uint,
    points: *const f64, pointcount: c_uint,
    polygons: *const c_uint) {
unsafe {
    dsDrawConvexD(pos, rot, planes, planecount, points, pointcount, polygons);
}
  }
  /// dsDrawCylinder 3 12
  fn DrawCylinder(&self, pos: *const f32, rot: *const f32, l: f32, r: f32) {
unsafe {
    dsDrawCylinder(pos, rot, l, r);
}
  }
  /// dsDrawCylinderD 3 12
  fn DrawCylinderD(&self, pos: *const f64, rot: *const f64, l: f32, r: f32) {
unsafe {
    dsDrawCylinderD(pos, rot, l, r);
}
  }
  /// dsDrawLine 3 3
  fn DrawLine(&self, pos1: *const f32, pos2: *const f32) {
unsafe {
    dsDrawLine(pos1, pos2);
}
  }
  /// dsDrawLineD 3 3
  fn DrawLineD(&self, pos1: *const f64, pos2: *const f64) {
unsafe {
    dsDrawLineD(pos1, pos2);
}
  }
  /// dsDrawSphere 3 12
  fn DrawSphere(&self, pos: *const f32, rot: *const f32, radius: f32) {
unsafe {
    dsDrawSphere(pos, rot, radius);
}
  }
  /// dsDrawSphereD 3 12
  fn DrawSphereD(&self, pos: *const f64, rot: *const f64, radius: f32) {
unsafe {
    dsDrawSphereD(pos, rot, radius);
}
  }
  /// dsDrawTriangle 3 12 (4 4 4) or (3 3 3)
  fn DrawTriangle(&self, pos: *const f32, rot: *const f32,
    v0: *const f32, v1: *const f32, v2: *const f32, solid: c_int) {
unsafe {
    dsDrawTriangle(pos, rot, v0, v1, v2, solid);
}
  }
  /// dsDrawTriangleD 3 12 (4 4 4) or (3 3 3)
  fn DrawTriangleD(&self, pos: *const f64, rot: *const f64,
    v0: *const f64, v1: *const f64, v2: *const f64, solid: c_int) {
unsafe {
    dsDrawTriangleD(pos, rot, v0, v1, v2, solid);
}
  }
  /// dsDrawTriangles 3 12 4n or 3n
  fn DrawTriangles(&self, pos: *const f32, rot: *const f32,
    v: *const f32, n: c_int, solid: c_int) {
unsafe {
    dsDrawTriangles(pos, rot, v, n, solid);
}
  }
  /// dsDrawTrianglesD 3 12 4n or 3n
  fn DrawTrianglesD(&self, pos: *const f64, rot: *const f64,
    v: *const f64, n: c_int, solid: c_int) {
unsafe {
    dsDrawTrianglesD(pos, rot, v, n, solid);
}
  }
  /// dsElapsedTime
  fn ElapsedTime(&self) -> f64 {
unsafe {
    dsElapsedTime()
}
  }
  /// dsGetViewpoint 3 3
  fn GetViewpoint(&self, xyz: *mut f32, hpr: *mut f32) {
unsafe {
    dsGetViewpoint(xyz, hpr);
}
  }
  /// dsSetCapsuleQuality default 3
  fn SetCapsuleQuality(&self, n: c_int) {
unsafe {
    dsSetCapsuleQuality(n);
}
  }
  /// dsSetColor
  fn SetColor(&self, red: f32, green: f32, blue: f32) {
unsafe {
    dsSetColor(red, green, blue);
}
  }
  /// dsSetColorAlpha
  fn SetColorAlpha(&self, red: f32, green: f32, blue: f32, alpha: f32) {
unsafe {
    dsSetColorAlpha(red, green, blue, alpha);
}
  }
  /// dsSetDrawMode
  fn SetDrawMode(&self, mode: c_int) {
unsafe {
    dsSetDrawMode(mode);
}
  }
  /// dsSetSphereQuality default 1
  fn SetSphereQuality(&self, n: c_int) {
unsafe {
    dsSetSphereQuality(n);
}
  }
  /// dsSetTexture
  fn SetTexture(&self, texture_number: c_int) {
unsafe {
    dsSetTexture(texture_number);
}
  }
  /// dsSetViewpoint 3 3
  fn SetViewpoint(&self, xyz: *mut f32, hpr: *mut f32) {
unsafe {
    dsSetViewpoint(xyz, hpr);
}
  }
  /// dsSimulationLoop
  fn SimulationLoop(&self, argc: c_int, argv: *mut *mut c_char,
    window_width: c_int, window_height: c_int, functions: *mut dsFunctions_C) {
unsafe {
    dsSimulationLoop(argc, argv, window_width, window_height,
      functions as *mut dsFunctions); // (for compatibililty)
}
  }
  /// dsStop
  fn Stop(&self) {
unsafe {
    dsStop();
}
  }
}

/// Drawstuff
impl Drawstuff {
  /// constructor
  pub fn new() -> Self {
    Drawstuff{}
  }
  /// dispose
  pub fn dispose(&mut self) {
    ()
  }
}

/// Drop for Drawstuff
impl Drop for Drawstuff {
  /// drop
  fn drop(&mut self) {
    self.dispose();
  }
}
