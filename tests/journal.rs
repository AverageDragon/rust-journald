extern crate journald;

#[test]
fn test_reverse_walk() {
	let messages_expected = vec![
		"rust-systemd test 1",
		"rust-systemd test 2",
		"rust-systemd test 3",
	];

	for message in &messages_expected {
		journald::writer::send(&[&format!("MESSAGE={}", message)]);
	}

	let mut journal = journald::reader::JournalReader
			::open(journald::reader::JournalFiles::All, false, false)
			.expect("journal open failed");

	journal
			.seek(journald::reader::JournalSeek::Tail)
			.expect("journal seek failed");

	let mut messages_actual = Vec::<String>::new();

	for _ in 0..3 {
		let entry = journal
				.previous_entry()
				.expect("previous_record() failed");

		messages_actual.insert(0, entry.unwrap().get("MESSAGE").unwrap().to_string());
	}

	assert!(messages_expected == messages_actual);
}

