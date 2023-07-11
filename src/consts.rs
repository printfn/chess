include!(concat!(env!("OUT_DIR"), "/consts.rs"));

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn adjacent() {
		for i in 0..8 {
			for j in 0..8 {
				let mut expected = 0_u64;
				for x in 0..8 {
					for y in 0..8 {
						if i == x && y == j {
							continue;
						}
						if (i as i32 - x as i32).abs() <= 1 && (j as i32 - y as i32).abs() <= 1 {
							expected |= 1 << (x * 8 + y);
						}
					}
				}
				assert_eq!(ADJACENT_BITBOARDS[i * 8 + j], expected);
			}
		}
	}

	#[test]
	fn count_knight_moves() {
		let mut sum = 0;
		for x in KNIGHT_BITBOARDS {
			sum += x.count_ones();
		}
		// 4 corners * 2 moves = 8
		assert_eq!(KNIGHT_BITBOARDS[0].count_ones(), 2);
		// 8 corner/edges * 3 moves = 24
		assert_eq!(KNIGHT_BITBOARDS[1].count_ones(), 3);
		assert_eq!(KNIGHT_BITBOARDS[8].count_ones(), 3);
		// 16 middle edges * 4 moves = 64
		assert_eq!(KNIGHT_BITBOARDS[2].count_ones(), 4);
		assert_eq!(KNIGHT_BITBOARDS[3].count_ones(), 4);
		assert_eq!(KNIGHT_BITBOARDS[16].count_ones(), 4);
		assert_eq!(KNIGHT_BITBOARDS[24].count_ones(), 4);
		// 4 inset corners * 4 moves = 16
		assert_eq!(KNIGHT_BITBOARDS[9].count_ones(), 4);
		// 16 inset edges * 6 moves = 96
		assert_eq!(KNIGHT_BITBOARDS[10].count_ones(), 6);
		assert_eq!(KNIGHT_BITBOARDS[11].count_ones(), 6);
		assert_eq!(KNIGHT_BITBOARDS[17].count_ones(), 6);
		assert_eq!(KNIGHT_BITBOARDS[25].count_ones(), 6);
		// 16 middle * 8 moves = 128
		// total = 336
		assert_eq!(sum, 336);
	}
}
