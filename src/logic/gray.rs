/// Generate the gray code sequence for n variables
///
/// ### Examples
/// ```
/// let seq = gray_sequence(3);
/// assert_eq!(seq, vec![0, 1, 3, 2, 6, 7, 5, 4]);
/// ```
///
/// ### Parameters
/// - `n`: The number of bits (variables) in the Gray code.
///
/// ### Returns
/// A vector containing all `2^n` Gray code values in order.
pub fn gray_sequence(n: u8) -> Vec<u8> {
    let size = 1u8 << n;
    (0..size).map(|i| i ^ (i >> 1)).collect()
}

/// Return a formatted string representing a gray code
/// ### Examples
/// ```
/// assert_eq!(format_bits(1, 2), "01");
/// assert_eq!(format_bits(5, 4), "0101");
/// ```
///
/// ### Parameters
/// - `gray`: The integer value to format.
/// - `width`: The total number of bits to display.
pub fn format_bits(gray: u8, width: u8) -> String {
    format!("{:0width$b}", gray, width = width as usize)
}

pub fn gray_to_binary(mut g: u8) -> u8 {
    g ^= g >> 1;
    g ^= g >> 2;
    g ^= g >> 4;
    g
}

/// Computes the K-map coordinates for a given minterm.
///
/// Given a minterm value in binary form and the number of bits assigned to
/// the row and column groups, this function returns the `(row_index, col_index)`
/// corresponding to the K-map position. The indices are based on Gray code order
///
/// ### Parameters
/// - `minterm`: The binary minterm value (e.g., 0–15 for 4-variable maps).
/// - `row_bits`: Number of bits assigned to the K-map rows (e.g., `3` for `ABC`).
/// - `col_bits`: Number of bits assigned to the K-map columns (e.g., `2` for `DE`).
///
/// ### Returns
/// A tuple `(row_index, col_index)` giving the K-map cell position.
///
/// ### Examples
/// ```
/// // 5-variable map split as ABC / DE
/// let (row, col) = extract_row_col(12, 3, 2);
/// assert_eq!((row, col), (4, 0));
///
/// // 4-variable map split as AB / CD
/// let (row, col) = extract_row_col(6, 2, 2);
/// assert_eq!((row, col), (1, 3));
/// ```
pub fn extract_row_col(minterm: u8, row_bits: usize, col_bits: usize) -> (u8, u8) {
    let col_mask: u8 = if col_bits == 0 {
        0
    } else {
        (1u16.wrapping_shl(col_bits as u32) - 1) as u8
    };

    let row_mask: u8 = if row_bits == 0 {
        0
    } else {
        (1u16.wrapping_shl(row_bits as u32) - 1) as u8
    };

    let col_gray = minterm & col_mask;
    let row_gray = (minterm >> col_bits) & row_mask;

    let row_idx = gray_to_binary(row_gray);
    let col_idx = gray_to_binary(col_gray);
    (row_idx, col_idx)
}

#[test]
fn can_extract_two_ab() {
    let expected = vec![(0, 0), (0, 1), (1, 0), (1, 1)];

    let result: Vec<(u8, u8)> = (0..4).map(|m| extract_row_col(m, 1, 1)).collect();
    assert_eq!(expected, result);
}

#[test]
fn can_extract_three_ab_c() {
    let expected: Vec<(u8, u8)> = vec![
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (3, 0),
        (3, 1),
        (2, 0),
        (2, 1),
    ];

    let result: Vec<(u8, u8)> = (0..8).map(|m| extract_row_col(m, 2, 1)).collect();
    assert_eq!(expected, result);
}

#[test]
fn can_extract_three_a_bc() {
    let expected: Vec<(u8, u8)> = vec![
        (0, 0),
        (0, 1),
        (0, 3),
        (0, 2),
        (1, 0),
        (1, 1),
        (1, 3),
        (1, 2),
    ];

    let result: Vec<(u8, u8)> = (0..8).map(|m| extract_row_col(m, 1, 2)).collect();
    assert_eq!(expected, result);
}

#[test]
fn can_extract_four() {
    let expected: Vec<(u8, u8)> = vec![
        (0, 0),
        (0, 1),
        (0, 3),
        (0, 2),
        (1, 0),
        (1, 1),
        (1, 3),
        (1, 2),
        (3, 0),
        (3, 1),
        (3, 3),
        (3, 2),
        (2, 0),
        (2, 1),
        (2, 3),
        (2, 2),
    ];

    let result: Vec<(u8, u8)> = (0..16).map(|m: u8| extract_row_col(m, 2, 2)).collect();

    assert_eq!(expected, result);
}
