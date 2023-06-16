// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

fn main() {
    prost_build::compile_protos(&["src/messages.proto"], &["src/"]).unwrap();
}
