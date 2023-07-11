use std::{env, fs, path};

fn generate_bitboards() -> String {
	let mut result = "pub const ORTHOGONAL_BITBOARDS: [u64; 64] = [\n".to_string();
	for i in 0..64 {
		let mut value = 0_u64;
		for j in 0..64 {
			if i != j && (i / 8 == j / 8 || i % 8 == j % 8) {
				value |= 1 << j;
			}
		}
		result.push_str(&format!("\t0x{value:016x},\n"));
	}
	result.push_str("];\n");

	result.push_str("pub const DIAGONAL_BITBOARDS: [u64; 64] = [\n");
	for i in 0..64 {
		let mut value = 0_u64;
		for j in 0..64 {
			if i != j && (i / 8 + i % 8 == j / 8 + j % 8 || i / 8 + 7 - i % 8 == j / 8 + 7 - j % 8)
			{
				value |= 1 << j;
			}
		}
		result.push_str(&format!("\t0x{value:016x},\n"));
	}
	result.push_str("];\n");

	result.push_str("pub const ADJACENT_BITBOARDS: [u64; 64] = [\n");
	for i in 0..64 {
		let mut value = 0_u64;
		if i < 56 {
			value |= 1 << (i + 8);
		}
		if i >= 8 {
			value |= 1 << (i - 8);
		}
		if i % 8 != 0 {
			value |= 1 << (i - 1);
		}
		if i % 8 != 7 {
			value |= 1 << (i + 1);
		}
		if i < 56 {
			if i % 8 != 0 {
				value |= 1 << (i + 7);
			}
			if i % 8 != 7 {
				value |= 1 << (i + 9);
			}
		}
		if i >= 8 {
			if i % 8 != 0 {
				value |= 1 << (i - 9);
			}
			if i % 8 != 7 {
				value |= 1 << (i - 7);
			}
		}
		result.push_str(&format!("\t0x{value:016x},\n"));
	}
	result.push_str("];\n");

	result.push_str("pub const KNIGHT_BITBOARDS: [u64; 64] = [\n");
	for i in 0..64 {
		let mut value = 0_u64;
		if i < 48 {
			if i % 8 != 0 {
				value |= 1 << (i + 15);
			}
			if i % 8 != 7 {
				value |= 1 << (i + 17);
			}
		}
		if i < 56 {
			if i % 8 >= 2 {
				value |= 1 << (i + 6);
			}
			if i % 8 <= 5 {
				value |= 1 << (i + 10);
			}
		}
		if i >= 16 {
			if i % 8 != 7 {
				value |= 1 << (i - 15);
			}
			if i % 8 != 0 {
				value |= 1 << (i - 17);
			}
		}
		if i >= 8 {
			if i % 8 >= 2 {
				value |= 1 << (i - 10);
			}
			if i % 8 <= 5 {
				value |= 1 << (i - 6);
			}
		}
		result.push_str(&format!("\t0x{value:016x},\n"));
	}
	result.push_str("];\n");
	result
}

fn main() {
	println!("cargo:rerun-if-changed=build.rs");

	let out_dir = env::var_os("OUT_DIR").unwrap();
	let dest_path = path::Path::new(&out_dir).join("consts.rs");
	fs::write(&dest_path, generate_bitboards()).unwrap();
}
