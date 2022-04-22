use clap:: {
	Args,
	Parser,
	Subcommand
};

/* 
	Example of a simple parser.
	Triple slash comments are used in help system!
*/

/*
#[derive(Debug, Parser)]
#[clap(author="Dimni", 
	version="0.01", 
	about="A simple argument parsing example", 
	long_about="A simple argument parsing example, but this about should be longer :)")]
pub struct Arguments{
	/// First argument help
	pub first_argument: String,

	/// Second argument help
	pub second_argument: String,

	/// Third argument help
	pub third_argument: String,

	/// Fourth argument help
	pub fourth_argument: u32,	
}
*/

#[derive(Debug, Parser)]
#[clap(author="Dimni", 
	version="0.01", 
	about="A argument parsing example, ", 
	long_about="An argument parsing example, featuring subcommands, long_about version")]
pub struct Arguments{
	#[clap(subcommand)]
	pub entity_type: SubcommandTree,
}

#[derive(Debug, Subcommand)]
pub enum SubcommandTree{
	/// First command help
	FirstCommand(FirstCommandHandler),

	/// Second command help
	SecondCommand(SecondCommandHandler)
}

#[derive(Debug, Args)]
pub struct FirstCommandHandler{
	#[clap(subcommand)]
	pub command: FirstSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum FirstSubcommand{
	/// First subcommand help
	FirstSubcommand(FirstSubcommandHandler),

	/// Second subcommand help
	SecondSubcommand(SecondSubcommandHandler),

	/// Third subcommand help
	ThirdSubcommand(ThirdSubcommandHandler),
}

#[derive(Debug, Args)]
pub struct FirstSubcommandHandler{
	/// First argument help
	#[clap(short, long)] // short adds a short flag (-f), long adds a long flag (--first-argument)
						 // can be customised by short='v' for example
	pub first_argument: String,

	/// Second argument help
	pub second_argument: Option<String>, // optional argument!
}

#[derive(Debug, Args)]
pub struct SecondSubcommandHandler{
	/// First argument help
	pub first_argument: String,

	/// Second argument help
	pub second_argument: String,
}

#[derive(Debug, Args)]
pub struct ThirdSubcommandHandler{
	/// First argument help
	pub first_argument: String,

	/// Second argument help
	pub second_argument: String,
}

#[derive(Debug, Args)]
pub struct SecondCommandHandler{

	/// First argument help
	pub first_argument: String,

	/// Second argument help
	pub second_argument: String,	

	/// Third argument help
	pub third_argument: String,
}




