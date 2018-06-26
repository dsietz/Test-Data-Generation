//! The `data_sample_parser` module provides functionality to read sample data, parse and analyze it,
//! so that test data can be generated based on profiles.
//!
//! # Examples
//!
//!
//! Generate some demo test data ...
//!
//! ```
//! extern crate test_data_generation;
//!
//! use test_data_generation::data_sample_parser::DataSampleParser;
//!
//! fn main() {
//!		// initalize a new DataSampelParser
//!		let dsp = DataSampleParser::new();
//!
//!		// generate some test data using the demo functions
//!		println!("generate date:{}", dsp.demo_date());
//!		println!("generate person:{}", dsp.demo_person_name());
//! }
//! ```
//!
//! Save the algorithm ...
//!
//! Archive (export) the data sample parser object so that you can reuse the algorithm to generate test data at a later time.
//! This enables you to persist the algorithm without having to store the actual data sample that was used to create the algorithm -
//! Which is important if you used 'real' data in your sample data.
//!
//! ```
//! extern crate test_data_generation;
//!
//! use test_data_generation::data_sample_parser::DataSampleParser;
//!
//! fn main() {
//! 	// analyze the dataset
//!		let mut dsp =  DataSampleParser::new();
//!
//!     assert_eq!(dsp.save(&String::from("./tests/samples/empty-dsp")).unwrap(), true);
//! }
//! ```
//!
//! Load an algorithm ...
//!
//! Create a data sample parser from a previously saved (exported) archive file so you can generate test data based on the algorithm.</br>
//! *NOTE:* In this example, there was only one data point in the data smaple that was analyzed (the word 'OK'). This was intentional
//! so the algorithm would be guaranteed to generate that same word. This was done ensure the assert_eq! returns true.
//!
//! ```
//! extern crate test_data_generation;
//!
//! use test_data_generation::data_sample_parser::DataSampleParser;
//!
//! fn main() {
//!		let mut dsp = DataSampleParser::from_file(&String::from("./tests/samples/sample-00-dsp"));
//!
//!		assert_eq!(dsp.generate_record()[0], "OK".to_string());
//! }
//! ```
//!
//! You can also generate a new csv file based on the data sample provided.
//!
//! ```
//! extern crate test_data_generation;
//!
//! use test_data_generation::data_sample_parser::DataSampleParser;
//!
//! fn main() {
//!     let mut dsp =  DataSampleParser::new();
//!
//!    	dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap();
//!    	dsp.generate_csv(100, &String::from("./tests/samples/generated-01.csv")).unwrap();
//! }
//! ```
//!

use std::collections::BTreeMap;
use configs::Configs;
use profile::profile::{Profile};
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::prelude::*;
use std::result::Result;
use csv;
use std::error::Error;
use csv::WriterBuilder;
use serde_json;
use levenshtein;

type ProfilesMap = BTreeMap<String, Profile>;

#[derive(Serialize, Deserialize, Debug)]
/// Represents the Parser for sample data to be used
pub struct DataSampleParser{
	/// indicates if there were issues parsing and anlyzing the data sample
	pub issues: bool,
	/// Configs object that define the configuration settings
	cfg: Option<Configs>,
	/// List of Profiles objects identified by a unique profile name BTreeMap<String, Profile>
	profiles: ProfilesMap,
}

impl DataSampleParser {
	/// Constructs a new DataSampleParser
	///
	/// #Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///
	/// fn main() {
	///		// initalize a new DataSampelParser
	///		let dsp = DataSampleParser::new();
	/// }
	/// ```
	pub fn new() -> DataSampleParser {

		DataSampleParser{
			issues: false,
            cfg: None,
            profiles: ProfilesMap::new(),
		}
	}

	/// Constructs a new DataSampleParser
	///
	/// # Arguments
	///
	/// * `path: &String - The full path name (including the file name and extension) to the configuration file.</br>
	///
	/// #Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///
	/// fn main() {
	///		// initalize a new DataSampelParser
	///	    // param: the path to the configuration  file
	///		let dsp = DataSampleParser::new_with(&String::from("./config/tdg.yaml"));
	/// }
	/// ```
	pub fn new_with(path: &String) -> DataSampleParser {
		DataSampleParser{
			issues: false,
            cfg: Some(Configs::new(path)),
            profiles: ProfilesMap::new(),
		}
	}

	/// Constructs a new DataSampleParser from an exported JSON file. This is used when restoring from "archive"
	///
	/// # Arguments
	///
	/// * `path: &String` - The full path name of the json formatted Data Sample Parser archive file.</br>
	///
	/// #Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///
	/// fn main() {
	///		let mut dsp = DataSampleParser::from_file(&String::from("./tests/samples/sample-00-dsp"));
    ///
    ///		assert_eq!(dsp.generate_record()[0], "OK".to_string());
	/// }
    /// ```
	pub fn from_file(path: &String) -> DataSampleParser {
		// open the archive file
		let mut file = match File::open(format!("{}.json",&path)) {
			Err(_e) => {
				error!("Could not open file {:?}", &path.to_string());
				panic!("Could not open file {:?}", &path.to_string());
			},
			Ok(f) => {
				info!("Successfully opened file {:?}", &path.to_string());
				f
			},
		};

		//read the archive file
		let mut serialized = String::new();
		match file.read_to_string(&mut serialized) {
			Err(e) => {
				error!("Could not read file {:?} because of {:?}", &path.to_string(), e.to_string());
				panic!("Could not read file {:?} because of {:?}", &path.to_string(), e.to_string());
			},
			Ok(s) => {
				info!("Successfully read file {:?}", &path.to_string());
				s
			},
		};

		serde_json::from_str(&serialized).unwrap()
	}

	/// This function analyzes sample data that is a csv formatted file and returns a boolean if successful.
	/// _NOTE:_ The csv properties are as follows:
	///       + headers are included as first line
	///       + double quote wrap text
	///       + double quote escapes is enabled
	///       + delimiter is a comma
	///
	///
	/// # Arguments
	///
	/// * `path: &String` - The full path name of the csv formatted sample data file.</br>
	///
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///
	/// fn main() {
	///		// initalize a new DataSampelParser
	///		let mut dsp = DataSampleParser::new();
    ///
    /// 	assert_eq!(dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap(),1);
	/// }
	/// ```
	pub fn analyze_csv_file(&mut self, path: &String) -> Result<i32, String>  {
		info!("Starting to analyzed the csv file {}",path);

    	let mut file = try!(File::open(path).map_err(|e| {
			error!("csv file {} couldn't be opened!",path);
    		e.to_string()
		}));

		let mut data = String::new();
    	file.read_to_string(&mut data).map_err(|e| {
			error!("csv file {} couldn't be read!",path);
    		e.to_string()
		}).unwrap();
		self.analyze_csv_data(&data)
	}

	/// This function analyzes sample data that is a csv formatted string and returns a boolean if successful.
	/// _NOTE:_ The csv properties are as follows:
	///       + headers are included as first line
	///       + double quote wrap text
	///       + double quote escapes is enabled
	///       + delimiter is a comma
	///
	///
	/// # Arguments
	///
	/// * `data: &String` - The textual content of a csv formatted sample data file.</br>
	///
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///
	/// fn main() {
	///		// initalize a new DataSampelParser
	///		let mut dsp = DataSampleParser::new();
	///		let mut data = String::from("");
	///		data.push_str("\"firstname\",\"lastname\"\n");
	///		data.push_str("\"Aaron\",\"Aaberg\"\n");
	///		data.push_str("\"Aaron\",\"Aaby\"\n");
	///		data.push_str("\"Abbey\",\"Aadland\"\n");
	///		data.push_str("\"Abbie\",\"Aagaard\"\n");
	///		data.push_str("\"Abby\",\"Aakre\"");
    ///
    /// 	assert_eq!(dsp.analyze_csv_data(&data).unwrap(),1);
	/// }
	/// ```
	pub fn analyze_csv_data(&mut self, data: &String) -> Result<i32, String>  {
		println!("{}",data);

		debug!("Starting to analyzed the csv data {}",data);

		let mut rdr = csv::ReaderBuilder::new()
        	.has_headers(true)
        	.quote(b'"')
        	.double_quote(true)
        	.delimiter(b',')
        	.from_reader(data.as_bytes());

		//iterate through the headers
		for headers in rdr.headers() {
			for header in headers.iter() {
	        	//add a Profile to the list of profiles to represent the field (indexed using the header label)
	        	let p = Profile::new();
	        	self.profiles.insert(format!("{}",header), p);
	        }
		}

		//create a Vec from all the keys (headers) in the profiles list
		let profile_keys: Vec<_> = self.profiles.keys().cloned().collect();
		let mut rec_cnt: u16 = <u16>::min_value();

		debug!("CSV headers: {:?}",profile_keys);
/*
		// Multi-Threading START
		let mut records_analysis = Vec::new();

		crossbeam::scope(|scope|{
			for row in rdr.records() {
				//keep a count of the number of records analyzed
	        	rec_cnt = rec_cnt + 1;

				let analysis = scope.spawn(move|| -> Vec<(String, Vec<Fact>)>{
					let record = row.expect("a CSV record");
					let mut record_analysis = Vec::new();

					//iterate through all the fields
	        		for (idx, field) in record.iter().enumerate() {
						let p = Profile::new();
						record_analysis.insert(idx, p.factualize(field));
					}

					record_analysis
				}).join();

				records_analysis.push(analysis);
			}
		});

		for analysis in records_analysis {
			for (idx, factual) in analysis.into_iter().enumerate()  {
				let pattern = factual.0;
				let facts = factual.1;
				self.profiles.get_mut(&profile_keys[idx]).unwrap().apply_facts(pattern, facts);
			}
		}
		// Multi-Threading END
*/

		// Single-Threading START
		//iterate through all the records
	    for result in rdr.records() {
	        let record = result.expect("a CSV record");

	        //keep a count of the number of records analyzed
	        rec_cnt = rec_cnt + 1;

	        //iterate through all the fields
	        for (idx, field) in record.iter().enumerate() {
	        	// Print a debug version of the record.
	        	debug!("Field Index: {}, Field Value: {}", idx, field);

	        	//select the profile based on the field name (header) and analyze the field value
	        	self.profiles.get_mut(&profile_keys[idx]).unwrap().analyze(field);
	        }
	    }
		// Single-Threading END

	    debug!("Successfully analyzed the csv data");
		debug!("Analyzed {} records, {} fields", rec_cnt, self.profiles.len());

		//prepare the profiles for data generation
		self.profiles.iter_mut().for_each(|p|p.1.pre_generate());

		Ok(1)
	}

	/// This function generates date as strings using the a `demo` profile
	///
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///
	/// fn main() {
	///		// initalize a new DataSampelParser
	///		let dsp = DataSampleParser::new();
	///
	///		// generate some test data using the demo functions
	///		println!("generate date:{}", dsp.demo_date());
	/// }
	/// ```
	pub fn demo_date(&self) -> String{
		let mut profil =  Profile::new();

    	profil.analyze("01/04/2017");
    	profil.analyze("02/09/2017");
    	profil.analyze("03/13/2017");
    	profil.analyze("04/17/2017");
    	profil.analyze("05/22/2017");
    	profil.analyze("07/26/2017");
    	profil.analyze("08/30/2017");
    	profil.analyze("09/07/2017");
    	profil.analyze("10/11/2017");
    	profil.analyze("11/15/2017");
    	profil.analyze("12/21/2017");
    	profil.analyze("01/14/2016");
    	profil.analyze("02/19/2016");
    	profil.analyze("03/23/2016");
    	profil.analyze("04/27/2016");
    	profil.analyze("05/02/2016");
    	profil.analyze("07/16/2015");
    	profil.analyze("08/20/2015");
    	profil.analyze("09/17/2015");
    	profil.analyze("10/01/2014");
    	profil.analyze("11/25/2014");
    	profil.analyze("12/31/2018");

    	profil.pre_generate();
    	//profil.apply_facts("##p##p####".to_string())
    	profil.generate()
	}

	/// This function generates people's names as strings using the a `demo` profile
	///
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///
	/// fn main() {
	///		// initalize a new DataSampelParser
	///		let dsp = DataSampleParser::new();
	///
	///		// generate some test data using the demo functions
	///		println!("generate date:{}", dsp.demo_person_name());
	/// }
	pub fn demo_person_name(&self) -> String{
	    let mut profil =  Profile::new();

    	profil.analyze("Smith, John");
    	profil.analyze("O'Brien, Henny");
    	profil.analyze("Dale, Danny");
    	profil.analyze("Rickets, Ronnae");
    	profil.analyze("Richard, Richie");
    	profil.analyze("Roberts, Blake");
    	profil.analyze("Conways, Sephen");

    	profil.pre_generate();
    	profil.generate()
	}

	/// This function returns a vector of header names
	///
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///
	/// fn main() {
	///		// initalize a new DataSampelParser
	///		let mut dsp = DataSampleParser::new();
    ///
    /// 	dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap();
    ///     let headers = dsp.extract_headers();
    ///
    ///		assert_eq!(headers.len(), 2);
	/// }
	pub fn extract_headers(&mut self) -> Vec<String> {
		let mut headers = vec!();

		for profile in self.profiles.iter_mut() {
			headers.push(profile.0.to_string());
		}

		headers
	}

	/// This function generates test data for the specified field name.
	///
	/// # Arguments
	///
	/// * `field: String` - The name of the field (e.g.: firstname) the represents the profile to use when generating the test data.</br>
	///
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///
	/// fn main() {
	///		// initalize a new DataSampelParser
	///		let mut dsp = DataSampleParser::new();
    ///
    /// 	dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap();
    ///     println!("Generated data for first name {}",dsp.generate_by_field_name("firstname".to_string()));
	/// }
	/// ```
	pub fn generate_by_field_name(&mut self, field: String) -> String {
		self.profiles.get_mut(&field).unwrap().generate().to_string()
	}

	/// This function Vec of generates test data fields.
	///
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///
	/// fn main() {
	///		// initalize a new DataSampelParser
	///		let mut dsp = DataSampleParser::new();
    ///
    /// 	dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap();
    ///     println!("Generated data record: {:?}",dsp.generate_record());
	/// }
	/// ```
	pub fn generate_record(&mut self) -> Vec<String> {
		let mut record = Vec::new();

		for profile in self.profiles.iter_mut() {
			record.push(profile.1.generate().to_string());
		}

		record
	}

	/// This function creates a csv file of generated test data.
	/// Prior to calling this funciton, you need to call the analyze_csv_file() function.
	/// _NOTE:_ The csv properties are as follows:
	///       + headers are included as first line
	///       + double quotes wrap text
	///       + double quote escapes is enabled
	///       + delimiter is a comma
	///
	///
	/// # Arguments
	///
	/// * `row_count: u32` - The number of rows to generate.</br>
	/// * `path: &String` - The full path name where to save the csv file.</br>
	///
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///
	/// fn main() {
	///		// initalize a new DataSampelParser
	///		let mut dsp = DataSampleParser::new();
    ///
    /// 	dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap();
    ///     dsp.generate_csv(100, &String::from("./tests/samples/generated-01.csv")).unwrap();
	/// }
	/// ```
	pub fn generate_csv(&mut self, row_count: u32, path: &String) -> Result<(), Box<Error>> {
		info!("generating csv file {}", path);

		let mut wtr = try!(WriterBuilder::new()
		    .has_headers(true)
        	.quote(b'"')
        	.double_quote(true)
        	.delimiter(b',')
        	.from_path(path).map_err(|e| {
			error!("csv file {} couldn't be created!",path);
    		e.to_string()
		}));

		let headers = self.extract_headers();
		wtr.write_record(&headers)?;

		for _r in 0..row_count {
			let mut record = Vec::new();

			for profile in self.profiles.iter_mut() {
				record.push(profile.1.generate());
			}

			wtr.write_record(&record)?;
		}

		wtr.flush()?;

		Ok(())
	}

	/// This function calculates the levenshtein distance between 2 strings.
	/// See: https://crates.io/crates/levenshtein
	///
	/// # Arguments
	///
	/// * `control: &String` - The string to compare against. This would be the real data from the data sample.</br>
	/// * `experiment: &String` - The string to compare. This would be the generated data for which you want to find the distance.</br>
	///
	/// #Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///
	/// fn main() {
	/// 	// analyze the dataset
	///		let mut dsp =  DataSampleParser::new();
	///
	///     assert_eq!(dsp.levenshtein_distance(&"kitten".to_string(), &"sitting".to_string()), 3 as usize);
	/// }
	///
	pub fn levenshtein_distance(&mut self, control: &String, experiment: &String) -> usize {
		// https://docs.rs/levenshtein/1.0.3/levenshtein/fn.levenshtein.html
		levenshtein::levenshtein(control, experiment)
	}

	/// This function calculates the percent difference between 2 strings.
	///
	/// # Arguments
	///
	/// * `control: &String` - The string to compare against. This would be the real data from the data sample.</br>
	/// * `experiment: &String` - The string to compare. This would be the generated data for which you want to find the percent difference.</br>
	///
	/// #Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///
	/// fn main() {
	/// 	// analyze the dataset
	///		let mut dsp =  DataSampleParser::new();
	///
	///     assert_eq!(dsp.realistic_test(&"kitten".to_string(), &"sitting".to_string()), 76.92307692307692 as f64);
	/// }
	///
	pub fn realistic_test(&mut self, control: &String, experiment: &String) -> f64 {
		//https://docs.rs/GSL/0.4.31/rgsl/statistics/fn.correlation.html
		//http://www.statisticshowto.com/probability-and-statistics/correlation-coefficient-formula/
		// pearson's chi square test
		// cosine similarity - http://blog.christianperone.com/2013/09/machine-learning-cosine-similarity-for-vector-space-models-part-iii/
		let ld: f64 = levenshtein::levenshtein(control, experiment) as f64;
		let total: f64 = control.len() as f64 + experiment.len() as f64;
		let diff: f64 = total - ld;
		(1 as f64 - ((total - diff)/total)) * 100   as f64
	}	

	/// This function returns a boolean that indicates if the data sample parsing had issues
	///
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///
	/// fn main() {
	///		// initalize a new DataSampelParser
	///	    // param: the path to the configuration file is wrong
	///		let dsp = DataSampleParser::new_with(&String::from("./target/debug/config/tdg.yaml"));
	///
	///		// generate some test data using the demo functions
	///		assert_eq!(dsp.running_with_issues(), &false);
	/// }
	pub fn running_with_issues(&self) -> &bool{
		&self.issues
	}

	/// This function saves (exports) the DataSampleParser to a JSON file.
	/// This is useful when you wish to reuse the algorithm to generate more test data later.
	///
	/// # Arguments
	///
	/// * `field: &String` - The full path of the export file , excluding the file extension, (e.g.: "./test/data/custom-names").</br>
	///
	/// #Errors
	/// If this function encounters any form of I/O or other error, an error variant will be returned.
	/// Otherwise, the function returns Ok(true).</br>
	///
	/// #Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///
	/// fn main() {
	/// 	// analyze the dataset
	///		let mut dsp =  DataSampleParser::new();
	///     dsp.analyze_csv_file(&String::from("./tests/samples/sample-00.csv")).unwrap();
	///
    ///     assert_eq!(dsp.save(&String::from("./tests/samples/sample-00-dsp")).unwrap(), true);
	/// }
	///
	pub fn save(&mut self, path: &String) -> Result<(bool), io::Error>  {
		let dsp_json = serde_json::to_string(&self).unwrap();

		// Create the archive file
		let mut file = match File::create(format!("{}.json",&path)) {
			Err(e) => {
				error!("Could not create file {:?}", &path.to_string());
				return Err(e);
			},
			Ok(f) => {
				info!("Successfully exported to {:?}", &path.to_string());
				f
			},
		};

		// Write the json string to file, returns io::Result<()>
    	match file.write_all(dsp_json.as_bytes()) {
        	Err(e) => {
            	error!("Could not write to file {}", &path.to_string());
            	return Err(e);
        	},
        	Ok(_) => {
        		info!("Successfully exported to {}", &path.to_string());
        	},
    	};

		Ok(true)
	}
}
