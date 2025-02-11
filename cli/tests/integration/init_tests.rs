// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

use test_util as util;
use util::assert_contains;
use util::TestContextBuilder;

#[test]
fn init_subcommand_without_dir() {
  let context = TestContextBuilder::new().use_temp_cwd().build();
  let cwd = context.temp_dir().path();

  let output = context.new_command().args("init").split_output().run();

  output.assert_exit_code(0);

  let stderr = output.stderr();
  assert_contains!(stderr, "Project initialized");
  assert!(!stderr.contains("cd"));
  assert_contains!(stderr, "deno run main.ts");
  assert_contains!(stderr, "deno task dev");
  assert_contains!(stderr, "deno test");

  assert!(cwd.join("deno.json").exists());

  let output = context
    .new_command()
    .env("NO_COLOR", "1")
    .args("run main.ts")
    .split_output()
    .run();

  output.assert_exit_code(0);
  assert_eq!(output.stdout().as_bytes(), b"Add 2 + 3 = 5\n");

  let output = context
    .new_command()
    .env("NO_COLOR", "1")
    .args("test")
    .split_output()
    .run();

  output.assert_exit_code(0);
  assert_contains!(output.stdout(), "1 passed");
  output.skip_output_check();
}

#[test]
fn init_subcommand_with_dir_arg() {
  let context = TestContextBuilder::new().use_temp_cwd().build();
  let cwd = context.temp_dir().path();

  let output = context
    .new_command()
    .args("init my_dir")
    .split_output()
    .run();

  output.assert_exit_code(0);

  let stderr = output.stderr();
  assert_contains!(stderr, "Project initialized");
  assert_contains!(stderr, "cd my_dir");
  assert_contains!(stderr, "deno run main.ts");
  assert_contains!(stderr, "deno task dev");
  assert_contains!(stderr, "deno test");

  assert!(cwd.join("my_dir/deno.json").exists());

  let output = context
    .new_command()
    .env("NO_COLOR", "1")
    .args("run my_dir/main.ts")
    .split_output()
    .run();

  output.assert_exit_code(0);

  assert_eq!(output.stdout().as_bytes(), b"Add 2 + 3 = 5\n");
  output.skip_output_check();

  let output = context
    .new_command()
    .env("NO_COLOR", "1")
    .args("test my_dir/main_test.ts")
    .split_output()
    .run();

  output.assert_exit_code(0);
  assert_contains!(output.stdout(), "1 passed");
  output.skip_output_check();
}

#[test]
fn init_subcommand_with_quiet_arg() {
  let context = TestContextBuilder::new().use_temp_cwd().build();
  let cwd = context.temp_dir().path();

  let output = context
    .new_command()
    .args("init --quiet")
    .split_output()
    .run();

  output.assert_exit_code(0);

  assert_eq!(output.stdout(), "");
  assert!(cwd.join("deno.json").exists());

  let output = context
    .new_command()
    .env("NO_COLOR", "1")
    .args("run main.ts")
    .split_output()
    .run();

  output.assert_exit_code(0);
  assert_eq!(output.stdout().as_bytes(), b"Add 2 + 3 = 5\n");
  output.skip_output_check();

  let output = context
    .new_command()
    .env("NO_COLOR", "1")
    .args("test")
    .split_output()
    .run();

  output.assert_exit_code(0);
  assert_contains!(output.stdout(), "1 passed");
  output.skip_output_check();
}
