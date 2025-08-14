watch . { |operation, path|
	match ($path | path parse | get extension) {
		"slint" | "nu" | "rs" => {
			if $operation == "Write" {
				let maybe_pid = ps | where name == "stickerbox" | get --optional pid.0;


				if $maybe_pid != null {
					kill $maybe_pid;
				}

				job spawn { cargo run; }
			}
		}
		_ => {}
	}
}
