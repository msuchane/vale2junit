/*
   Copyright 2023 Marek Suchánek

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

mod cli;
mod report;
mod vale;

fn main() {
    let args = cli::arguments();
    println!("Hello, world!");

    let json = match args.variant {
        cli::Variants::Input { input } => input,
        cli::Variants::File { file } => {
            std::fs::read_to_string(file).unwrap()
        }
    };

    let deserialized: vale::Alerts = serde_json::from_str(&json).unwrap();
    println!("{:#?}", deserialized);
    let report = report::junit_report(deserialized);
    println!("{:#?}", report);
}
