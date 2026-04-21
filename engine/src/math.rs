/// Column-major 4×4 matrix stored as a flat [f32; 16].
/// Index convention: [col * 4 + row]
pub type Mat4 = [f32; 16];

pub fn mat4_mul(a: &Mat4, b: &Mat4) -> Mat4 {
    let mut out = [0f32; 16];
    for col in 0..4usize {
        for row in 0..4usize {
            let mut sum = 0f32;
            for k in 0..4usize {
                sum += a[k * 4 + row] * b[col * 4 + k];
            }
            out[col * 4 + row] = sum;
        }
    }
    out
}

/// Right-hand perspective projection, depth range [0,1] (Vulkan / WebGPU).
pub fn perspective(fov_y_rad: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let f = 1.0 / (fov_y_rad * 0.5).tan();
    let rng = 1.0 / (near - far);
    [
        f / aspect, 0.0, 0.0,           0.0,
        0.0,        f,   0.0,           0.0,
        0.0,        0.0, far * rng,    -1.0,
        0.0,        0.0, near * far * rng, 0.0,
    ]
}

/// Rotation around the Y axis.
pub fn rotation_y(a: f32) -> Mat4 {
    let (s, c) = a.sin_cos();
    [
        c,   0.0, -s,  0.0,
        0.0, 1.0, 0.0, 0.0,
        s,   0.0, c,   0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}

/// Rotation around the X axis.
pub fn rotation_x(a: f32) -> Mat4 {
    let (s, c) = a.sin_cos();
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, c,   s,   0.0,
        0.0, -s,  c,   0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}

/// Translation matrix.
pub fn translation(x: f32, y: f32, z: f32) -> Mat4 {
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        x,   y,   z,   1.0,
    ]
}

/// Build MVP = Projection × View × Model (Y-rotation + fixed X-tilt).
pub fn compute_mvp(angle: f32, aspect: f32) -> Mat4 {
    let proj  = perspective(std::f32::consts::FRAC_PI_4, aspect, 0.1, 100.0);
    let view  = translation(0.0, 0.0, -3.0);
    let rot_y = rotation_y(angle);
    let rot_x = rotation_x(0.4); // slight downward tilt so the top face is visible

    let model    = mat4_mul(&rot_y, &rot_x);
    let view_mod = mat4_mul(&view, &model);
    mat4_mul(&proj, &view_mod)
}
