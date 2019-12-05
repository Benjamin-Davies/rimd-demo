use std::path::Path;

use rimd::*;

const MIDDLE_C: u8 = 60;
const BEAT: i16 = 96;
const BPM: f64 = 120.0;

fn add_note(builder: &mut SMFBuilder, beats: f64, note: u8) {
    builder.add_midi_rel(0, 0,
        MidiMessage::note_on(note, 100, 1));
    builder.add_midi_rel(0, (beats * BEAT as f64) as u64,
        MidiMessage::note_off(note, 100, 1));
}

fn main() {
    let mut builder = SMFBuilder::new();
    builder.add_track();

    // 4/4 time with sensible numbers for other fields
    // n beats per bar, 2^m beats per whole note
    builder.add_meta_abs(0, 0,
        MetaEvent::time_signature(4, 2, 24, 8));
    // Microseconds per quarter-note
    builder.add_meta_abs(0, 0,
        MetaEvent::tempo_setting((60_000_000.0 / BPM) as u32));

    add_note(&mut builder, 2.0, MIDDLE_C + 4);
    add_note(&mut builder, 1.0, MIDDLE_C + 5);
    add_note(&mut builder, 1.0, MIDDLE_C + 7);

    add_note(&mut builder, 1.0, MIDDLE_C + 7);
    add_note(&mut builder, 1.0, MIDDLE_C + 5);
    add_note(&mut builder, 1.0, MIDDLE_C + 4);
    add_note(&mut builder, 1.0, MIDDLE_C + 2);

    add_note(&mut builder, 1.0, MIDDLE_C + 0);
    add_note(&mut builder, 1.0, MIDDLE_C + 0);
    add_note(&mut builder, 1.0, MIDDLE_C + 2);
    add_note(&mut builder, 1.0, MIDDLE_C + 4);

    add_note(&mut builder, 1.5, MIDDLE_C + 4);
    add_note(&mut builder, 0.5, MIDDLE_C + 2);
    add_note(&mut builder, 2.0, MIDDLE_C + 2);

    let mut smf = builder.result();
    smf.division = BEAT;
    smf.format = SMFFormat::Single;
    let file_writer = SMFWriter::from_smf(smf);
    file_writer.write_to_file(Path::new("test.mid")).unwrap();
}
