use std::collections::HashMap;
use std::convert::TryInto;
use itertools::Itertools;

const BOARD_SIZE: usize = 9;
const PLAYER_NONE: u8 = 0;
const PLAYER_X: u8 = 1;
const PLAYER_O: u8 = 2;

// 012
// 345
// 678

// 630
// 741
// 852
pub fn rotated_mapping(mapping: &[i32; BOARD_SIZE], repeated_num: u32) -> [i32; BOARD_SIZE] {
    let mut rotated = [0; 9];
    let mut temp = mapping.clone();
    for _ in 0..repeated_num {
        rotated[0] = temp[6];
        rotated[1] = temp[3];
        rotated[2] = temp[0];
        rotated[3] = temp[7];
        rotated[4] = temp[4];
        rotated[5] = temp[1];
        rotated[6] = temp[8];
        rotated[7] = temp[5];
        rotated[8] = temp[2];
        temp.copy_from_slice(&rotated);
    }
    rotated
}

// 012
// 345
// 678

// 678
// 345
// 012

pub fn flipped_mapping(mapping: &[i32; BOARD_SIZE]) -> [i32; BOARD_SIZE] {
    let mut flipped = [0; BOARD_SIZE];
    flipped[0] = mapping[6];
    flipped[1] = mapping[7];
    flipped[2] = mapping[8];
    flipped[3] = mapping[3];
    flipped[4] = mapping[4];
    flipped[5] = mapping[5];
    flipped[6] = mapping[0];
    flipped[7] = mapping[1];
    flipped[8] = mapping[2];
    flipped
}

fn get_equivalent_mappings(mapping: &[i32; BOARD_SIZE]) -> [[i32; BOARD_SIZE]; 8] {
    [
        mapping.clone(),
        rotated_mapping(mapping, 1),
        rotated_mapping(mapping, 2),
        rotated_mapping(mapping, 3),
        flipped_mapping(mapping),
        rotated_mapping(&flipped_mapping(mapping), 1),
        rotated_mapping(&flipped_mapping(mapping), 2),
        rotated_mapping(&flipped_mapping(mapping), 3),
    ]
}

pub fn get_shift_map(initial_mapping: &[i32; BOARD_SIZE],
                     transform_mapping: &[i32; BOARD_SIZE]) -> HashMap<i32, u32> {
    let mut shift_map: HashMap<i32, u32> = HashMap::new();
    let offsets = initial_mapping.iter()
        .enumerate()
        .map(|(i, pos)| (i, i as i32 - transform_mapping.iter().position(|&p| p == *pos).unwrap() as i32));
    for (board_index, offset) in offsets.clone() {
        let shift = shift_map.entry(offset).or_insert(0);
        *shift |= 3u32 << (board_index * 2);
    }
    shift_map
}

fn get_all_shift_maps(mapping: &[i32; BOARD_SIZE]) -> Vec<HashMap<i32, u32>> {
    let mut shift_maps= vec![];
    for eq_mapping in get_equivalent_mappings(mapping).iter() {
        shift_maps.push(get_shift_map(mapping, &eq_mapping));
    }
    shift_maps
}

fn score_mapping(mapping: &[i32; BOARD_SIZE]) -> (f32, f32) {
    // println!("{:?}", mapping);
    let shift_maps = get_all_shift_maps(mapping);
    let num_shifts: Vec<f32> = shift_maps.iter().map(
        |sm| sm.len() as f32).collect();
    // println!("{:?}", num_shifts);
    let shift_totals = shift_maps.iter().map(|sm| sm.keys().sum::<i32>() as f32).collect_vec();
    let t: f32 = shift_totals.iter().map(|v| v.abs()).sum();
    (t + 9f32, variance(shift_totals))
}

fn variance(num_vec: Vec<f32>) -> f32 {
    let mean= num_vec.iter().sum::<f32>() / num_vec.len() as f32;
    let s = num_vec.iter().map(|n| (n - mean).powf(2.)).sum::<f32>();
    s / (num_vec.len() - 1) as f32
}

pub fn find_shift_optimized_mapping() -> [i32; BOARD_SIZE] {
    let mut min_shift_score = f32::INFINITY;
    let mut min_variance_score = f32::INFINITY;
    let mut best_mapping = [0; 9];
    for perm in (0..BOARD_SIZE as i32).permutations(BOARD_SIZE) {
        let arr_mapping: [i32; 9] = match perm.try_into() {
            Ok(arr) => arr,
            Err(arr) => panic!("Permutation {:?} is not the correct length", arr)
        };
        let (shift_score, variance_score) = score_mapping(&arr_mapping);
        println!("{} {} {:?}", shift_score, variance_score, &arr_mapping);
        if (shift_score < min_shift_score) ||
            (shift_score == min_shift_score && variance_score < min_variance_score) {
            min_shift_score = shift_score;
            min_variance_score = variance_score;
            best_mapping = arr_mapping;
        }
    }
    return best_mapping
}

pub fn const_str(name: &str, var_type: &str, value: &str) -> String {
    format!("pub const {}: {} = {};", name, var_type, value)
}

pub fn codegen_is_win_masks(mapping: &[i32; BOARD_SIZE]) -> String {
    let winning_positions = [
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],
        vec![0, 3, 6],
        vec![1, 4, 7],
        vec![2, 5, 8],
        vec![0, 4, 8],
        vec![2, 4, 6],
    ];
    let mut win_masks: [u32; 8] = [0; 8];
    for (i, positions) in winning_positions.iter().enumerate() {
        let mut pos_mask = 0;
        for pos in positions {
            let index = mapping[*pos as usize] as u32;
            pos_mask |= (PLAYER_X as u32) << (index * 2);

        }
        win_masks[i] = pos_mask;
    }
    let win_str = format!("[{}]", win_masks.iter().map(|wm| format!("{:#020b}", wm)).join(", "));
    const_str("WIN_MASKS",
              format!("[u32; {}]", win_masks.len()).as_str(),
              win_str.as_str())

}


fn codegen_transform_shifts(mapping: &[i32; BOARD_SIZE]) -> String {
    let shifts = get_all_shift_maps(mapping);
    let mut transform_strs = vec![];
    for transforms in shifts.iter() {
        let inner_transform_str = transforms.iter()
            .map(|(k, v)| format!("({}, {:#020b})", *k, *v))
            .collect_vec()
            .join(", ");
        let transform_str = format!("&[{}]", inner_transform_str);
        transform_strs.push(transform_str);
    }
    let value = format!("[{}]", transform_strs.join(", "));
    let type_name = format!("[&[(i32, u32)]; {}]", transform_strs.len());
    const_str("TRANSFORM_SHIFTS", type_name.as_str(), value.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotated_mapping() {
        let test_cases = vec![
            (&[0, 1, 2, 3, 4, 5, 6, 7, 8], 1, [6, 3, 0, 7, 4, 1, 8, 5, 2]),
            (&[0, 1, 2, 3, 4, 5, 6, 7, 8], 2, [8, 7, 6, 5, 4, 3, 2, 1, 0]),
            (&[0, 1, 2, 3, 4, 5, 6, 7, 8], 3, [2, 5, 8, 1, 4, 7, 0, 3, 6]),
            (&[0, 1, 2, 3, 4, 5, 6, 7, 8], 4, [0, 1, 2, 3, 4, 5, 6, 7, 8]),
            (&[0, 1, 2, 3, 4, 5, 6, 7, 8], 5, [6, 3, 0, 7, 4, 1, 8, 5, 2]),
        ];
        for (initial_mapping, num_rotations, result_mapping) in test_cases {
            let mapping = rotated_mapping(initial_mapping, num_rotations);
            assert_eq!(mapping, result_mapping)
        }
    }

    #[test]
    fn test_flipped_mapping() {
        let test_cases = vec![
            (&[0, 1, 2, 3, 4, 5, 6, 7, 8], [6, 7, 8, 3, 4, 5, 0, 1, 2]),
            (&[8, 7, 4, 1, 0, 3, 2, 5, 6], [2, 5, 6, 1, 0, 3, 8, 7, 4]),
        ];
        for (initial_mapping, result_mapping) in test_cases {
            let mapping = flipped_mapping(initial_mapping);
            assert_eq!(mapping, result_mapping)
        }
    }
}



fn main() {
    let optimized_mapping = find_shift_optimized_mapping();
    let win_mask_str = codegen_is_win_masks(&optimized_mapping);
    let transform_shifts_str = codegen_transform_shifts(&optimized_mapping);
    let board_size_str = const_str("BOARD_SIZE", "usize", "9");
    let player_none_str = const_str("PLAYER_NONE", "u8", "0");
    let player_x_str = const_str("PLAYER_X", "u8", "1");
    let player_o_str = const_str("PLAYER_O", "u8", "2");
    let x_mask = const_str("X_BIT_MASK", "u32", "0b010101010101010101");
    let o_mask = const_str("O_BIT_MASK", "u32", "0b101010101010101010");
    let position_map =  const_str("POSITION_MAP", "[usize; BOARD_SIZE]",
                                  format!("{:?}", optimized_mapping).as_str());
    let consts_str = vec!(
                             player_none_str,
                             player_x_str,
                             player_o_str,
                             transform_shifts_str,
                             win_mask_str,
                             board_size_str,
                             x_mask,
                             o_mask,
                             position_map).join("\n");
    println!("{}", consts_str)
}