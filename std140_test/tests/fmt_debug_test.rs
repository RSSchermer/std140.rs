#[test]
fn fmt_array_test() {
    assert_eq!(
        format!(
            "{:?}",
            std140::array![std140::float(0.0), std140::float(1.0),]
        ),
        "[float(0.0), float(1.0)]"
    );
}

#[test]
fn fmt_mat2x2_test() {
    assert_eq!(
        format!(
            "{:?}",
            std140::mat2x2(std140::vec2(1.0, 0.0), std140::vec2(0.0, 1.0),)
        ),
        "mat2x2[vec2(1.0, 0.0), vec2(0.0, 1.0)]"
    );
}

#[test]
fn fmt_mat2x3_test() {
    assert_eq!(
        format!(
            "{:?}",
            std140::mat2x3(std140::vec3(1.0, 0.0, 0.0), std140::vec3(0.0, 1.0, 0.0),)
        ),
        "mat2x3[vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0)]"
    );
}

#[test]
fn fmt_mat2x4_test() {
    assert_eq!(
        format!(
            "{:?}",
            std140::mat2x4(
                std140::vec4(1.0, 0.0, 0.0, 0.0),
                std140::vec4(0.0, 1.0, 0.0, 0.0),
            )
        ),
        "mat2x4[vec4(1.0, 0.0, 0.0, 0.0), vec4(0.0, 1.0, 0.0, 0.0)]"
    );
}

#[test]
fn fmt_mat3x2_test() {
    assert_eq!(
        format!(
            "{:?}",
            std140::mat3x2(
                std140::vec2(1.0, 0.0),
                std140::vec2(0.0, 1.0),
                std140::vec2(0.0, 0.0),
            )
        ),
        "mat3x2[vec2(1.0, 0.0), vec2(0.0, 1.0), vec2(0.0, 0.0)]"
    );
}

#[test]
fn fmt_mat3x3_test() {
    assert_eq!(
        format!(
            "{:?}",
            std140::mat3x3(
                std140::vec3(1.0, 0.0, 0.0),
                std140::vec3(0.0, 1.0, 0.0),
                std140::vec3(0.0, 0.0, 1.0),
            )
        ),
        "mat3x3[vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0)]"
    );
}

#[test]
fn fmt_mat3x4_test() {
    assert_eq!(
        format!(
            "{:?}",
            std140::mat3x4(
                std140::vec4(1.0, 0.0, 0.0, 0.0),
                std140::vec4(0.0, 1.0, 0.0, 0.0),
                std140::vec4(0.0, 0.0, 1.0, 0.0),
            )
        ),
        "mat3x4[vec4(1.0, 0.0, 0.0, 0.0), vec4(0.0, 1.0, 0.0, 0.0), vec4(0.0, 0.0, 1.0, 0.0)]"
    );
}

#[test]
fn fmt_mat4x2_test() {
    assert_eq!(
        format!(
            "{:?}",
            std140::mat4x2(
                std140::vec2(1.0, 0.0),
                std140::vec2(0.0, 1.0),
                std140::vec2(0.0, 0.0),
                std140::vec2(0.0, 0.0),
            )
        ),
        "mat4x2[vec2(1.0, 0.0), vec2(0.0, 1.0), vec2(0.0, 0.0), vec2(0.0, 0.0)]"
    );
}

#[test]
fn fmt_mat4x3_test() {
    assert_eq!(format!("{:?}", std140::mat4x3(
        std140::vec3(1.0, 0.0, 0.0),
        std140::vec3(0.0, 1.0, 0.0),
        std140::vec3(0.0, 0.0, 1.0),
        std140::vec3(0.0, 0.0, 0.0),
    )), "mat4x3[vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 0.0)]");
}

#[test]
fn fmt_mat4x4_test() {
    assert_eq!(format!("{:?}", std140::mat4x4(
        std140::vec4(1.0, 0.0, 0.0, 0.0),
        std140::vec4(0.0, 1.0, 0.0, 0.0),
        std140::vec4(0.0, 0.0, 1.0, 0.0),
        std140::vec4(0.0, 0.0, 0.0, 1.0),
    )), "mat4x4[vec4(1.0, 0.0, 0.0, 0.0), vec4(0.0, 1.0, 0.0, 0.0), vec4(0.0, 0.0, 1.0, 0.0), vec4(0.0, 0.0, 0.0, 1.0)]");
}
