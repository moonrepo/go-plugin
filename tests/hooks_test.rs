// TODO: Enable these once proto 0.15 is out!

// #[test]
// fn sets_gobin_to_shell() {
//     let temp = create_empty_sandbox();
//     let profile = temp.path().join(".profile");

//     let mut cmd = create_proto_command(temp.path());

//     cmd.env("TEST_PROFILE", &profile)
//         .arg("install")
//         .arg("go")
//         .arg("1.20.0")
//         .assert()
//         .success();

//     let output = fs::read_to_string(profile).unwrap();

//     assert!(predicate::str::contains("GOBIN=\"$HOME/go/bin\"").eval(&output));
// }

// #[test]
// fn doesnt_set_gobin() {
//     let temp = create_empty_sandbox();
//     let profile = temp.path().join(".profile");

//     let mut cmd = create_proto_command(temp.path());

//     cmd.env("TEST_PROFILE", &profile)
//         .arg("install")
//         .arg("go")
//         .arg("1.20.0")
//         .arg("--")
//         .arg("--no-gobin")
//         .assert()
//         .success();

//     assert!(!profile.exists());
// }
