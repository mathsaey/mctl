use led_matrix_serial_api::{LedMatrix, Result};

fn pattern(matrix: &mut LedMatrix, coords: Vec<(usize, Vec<usize>)>, brightness: u8) -> Result<()> {
    let mut pattern = [[0; 34]; 9];

    for (y, xs) in coords {
        for x in xs {
            pattern[y][x] = brightness;
        }
    }
    matrix.draw_cols(&pattern)
}

pub fn draw_speaker_mute(matrix: &mut LedMatrix) -> Result<()> {
    pattern(
        matrix,
        vec![
            (1, (17..=19).collect()),
            (2, (16..=20).collect()),
            (3, (15..=21).collect()),
            (5, vec![17, 19]),
            (6, vec![18]),
            (7, vec![17, 19]),
        ],
        u8::MAX,
    )
}

pub fn draw_speaker_on(matrix: &mut LedMatrix) -> Result<()> {
    pattern(
        matrix,
        vec![
            (1, (17..=19).collect()),
            (2, (16..=20).collect()),
            (3, (15..=21).collect()),
            (5, vec![16, 18, 20]),
            (6, vec![15, 17, 19, 21]),
        ],
        u8::MAX,
    )
}
