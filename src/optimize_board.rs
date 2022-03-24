use std::collections::HashMap;

fn rotated_mapping(mapping: &[i32; 9], repeated_num: u32) -> &[i32; 9] {
    let mut rotated: [i32; 9];
    let mut temp = mapping.clone();
    for _ in 0..repeated_num {
        temp.copy_from_slice(&rotated);
        rotated[0] = temp[6];
        rotated[1] = temp[3];
        rotated[2] = temp[0];
        rotated[3] = temp[7];
        rotated[4] = temp[4];
        rotated[5] = temp[1];
        rotated[6] = temp[8];
        rotated[7] = temp[5];
        rotated[8] = temp[2];
    }
    rotated.as_ref()
}

fn flipped_mapping(mapping: &[i32; 9]) -> &[i32; 9] {
    let mut flipped: [i32; 9];
    flipped[0] = mapping[6];
    flipped[1] = mapping[7];
    flipped[2] = mapping[8];
    flipped[3] = mapping[3];
    flipped[4] = mapping[4];
    flipped[5] = mapping[5];
    flipped[6] = mapping[0];
    flipped[7] = mapping[1];
    flipped[8] = mapping[2];
    flipped.as_ref()
}

fn get_equivalent_mappings(mapping: &[i32; 9]) -> &[&[i32; 9]; 7] {
    &[
        mapping.clone(),
        rotated_mapping(mapping, 1),
        rotated_mapping(mapping, 2),
        rotated_mapping(mapping, 3),
        flipped_mapping(mapping),
        flipped_mapping(rotated_mapping(mapping, 1)),
        flipped_mapping(rotated_mapping(mapping, 2)),
        flipped_mapping(rotated_mapping(mapping, 3)),
    ]
}

fn generate_rotate_shifts(mapping: &[i32; 9], repeated_num: u32) -> HashMap<i32, u32> {
    let mut shift_map: HashMap<i32, u32> = HashMap::new();
    let offsets = rotated_mapping(mapping, repeated_num)
        .iter()
        .enumerate()
        .map(|(i, pos)| (i, pos - mapping[i]));
    for (i, offset) in offsets {
        let shift = shift_map.entry(offset).or_insert(0);
        *shift & (3 >> i);
    }
    shift_map
}

fn main() {
    println!(generate_rotate_shifts(&[0, 1, 2, 3, 4, 5, 6, 7, 8], 1));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rotate_shifts() {
        let mapping = &[0, 1, 2, 3, 4, 5, 6, 7, 8];
        let shifts = super::generate_rotate_shifts(mapping, 1);

    }
}
