use raylib::prelude::*;


pub fn vector_sub(vector1: Vector2, vector2: Vector2) -> Vector2{
    return Vector2::new(vector1.x - vector2.x, vector1.y - vector2.y)
}

pub fn vector_add(vector1: Vector2, vector2: Vector2) -> Vector2{
    return Vector2::new(vector1.x + vector2.x, vector1.y + vector2.y)
}

pub fn vector_mult(vector1: Vector2, vector2: Vector2) -> Vector2{
    return Vector2::new(vector1.x * vector2.x, vector1.y * vector2.y)
}

pub fn set_magnitude(mut vector: Vector2, amount: f32) -> Vector2{
    vector.normalize();
    vector.scale(amount);
    return vector
}

pub fn set_limit(mut vector: Vector2, max: f32) -> Vector2{
    let mag_sq: f32 = vector.length_sqr();
    if mag_sq > max * max {
        vector.normalize();
        vector = vector_mult(vector, Vector2::new(max, max));
    }
    return vector
}

pub fn constrain(n: f32, min: f32, max:f32) -> f32{
        if n >= min && n <= max { return n }
        else if n >= min && n >= max { return max }
        else if n <= min && n <= max { return min }
        else { return n }
}
