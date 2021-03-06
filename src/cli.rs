// Copyright 2019-2020 Wei Tang.
// This file is part of Kulupu.

// Kulupu is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Kulupu is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Kulupu.  If not, see <http://www.gnu.org/licenses/>.

use sc_cli::RunCmd;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Subcommand {
	#[structopt(flatten)]
	Base(sc_cli::Subcommand),

	#[structopt(name = "export-builtin-wasm", setting = structopt::clap::AppSettings::Hidden)]
	ExportBuiltinWasm(ExportBuiltinWasmCommand),

	#[structopt(name = "import-mining-key")]
	ImportMiningKey(ImportMiningKeyCommand),

	#[structopt(name = "generate-mining-key")]
	GenerateMiningKey(GenerateMiningKeyCommand),
}

#[derive(Debug, StructOpt)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[structopt(flatten)]
	pub run: RunCmd,

	#[structopt(long)]
	pub author: Option<String>,
	#[structopt(long)]
	pub threads: Option<usize>,
	#[structopt(long)]
	pub round: Option<u32>,
	#[structopt(long)]
	pub enable_polkadot_telemetry: bool,
	#[structopt(long)]
	pub no_donate: bool,
	#[structopt(long)]
	pub check_inherents_after: Option<u32>,
}

#[derive(Debug, StructOpt)]
pub struct ExportBuiltinWasmCommand {
	#[structopt()]
	pub folder: String,
}

#[derive(Debug, StructOpt)]
pub struct ImportMiningKeyCommand {
	#[structopt()]
	pub suri: String,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub shared_params: sc_cli::SharedParams,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub keystore_params: sc_cli::KeystoreParams,
}

impl sc_cli::CliConfiguration for ImportMiningKeyCommand {
	fn shared_params(&self) -> &sc_cli::SharedParams { &self.shared_params }
	fn keystore_params(&self) -> Option<&sc_cli::KeystoreParams> { Some(&self.keystore_params) }
}

#[derive(Debug, StructOpt)]
pub struct GenerateMiningKeyCommand {
	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub shared_params: sc_cli::SharedParams,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub keystore_params: sc_cli::KeystoreParams,
}

impl sc_cli::CliConfiguration for GenerateMiningKeyCommand {
	fn shared_params(&self) -> &sc_cli::SharedParams { &self.shared_params }
	fn keystore_params(&self) -> Option<&sc_cli::KeystoreParams> { Some(&self.keystore_params) }
}
