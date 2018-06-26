## Test Data Generation
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Coverage Status](https://coveralls.io/repos/github/dsietz/test-data-generation/badge.svg?branch=master)](https://coveralls.io/github/dsietz/test-data-generation?branch=master)
[![Docs.rs](https://docs.rs/test-data-generation/badge.svg)](https://docs.rs/test-data-generation)

Linux: [![Build Status](https://travis-ci.org/dsietz/test-data-generation.svg?branch=master)](https://travis-ci.org/dsietz/test-data-generation)
Windows: [![Build status](https://ci.appveyor.com/api/projects/status/uw58v5t8ynwj8s8o/branch/master?svg=true)](https://ci.appveyor.com/project/dsietz/test-data-generation/branch/master)

### Description
For software development teams who need realistic test data for testing their software, this Test Data Generation library is a light-weight module
that implements Markov decision process machine learning to quickly and easily profile sample data, create an algorithm, and produce representative test data without the need for
persistent data sources, data cleaning, or remote services. Unlike other solutions, this open source solution can be integrated into your test source code, or
wrapped into a web service or stand-alone utility.   

**PROBLEM**
</br>
In order to make test data represent production, (a.k.a. realistic) you need to perform one of the following:
+ load data from a production environment into the non-production environment, which requires ETL (e.g.: masking, obfuscation, etc.)
+ stand up a pre-loaded "profile" database that is randomly sampled, which requires preparing sample data from either another test data source
or production environment (option #1 above)

**SOLUTION**
</br>
 Incorporate this library in your software's testing source code by loading an algorithm from a previously analyzed data sample and generating
 test data during your tests runtime.

---

### Table of Contents
* [What's New](#whats-new)
* [About](#about)
* [Usage](#usage)
* [How to Contribute](#how-to-contribute)
* [License](#license)

## What's New

Here's whats new in 0.0.6:

* Removed obsolete module test_data_generation::data
* Added functionality to determine how realist the generate test data is compared to the sample data.
> - test_data_generation::data_sample_parser::DataSampleParser::levenshtein_distance()
> - test_data_generation::data_sample_parser::DataSampleParser::realistic_test()

## About

`test data generation` uses [Markov decision process](https://en.wikipedia.org/wiki/Markov_decision_process) machine learning to create algorithms that enable test data generation on the fly without the overhead
of test data databases, security data provisioning (e.g.: masking, obfuscation), or standing up remote services.

The algorithm is built on the bases of:
1. character patterns
2. frequency of patterns
3. character locations
4. beginning and ending characters
5. length of entity (string, date, number)

## Usage

The are multiple ways to use the Test Data Generation library. It all depends on your intent.

### Profile

The easiest way is to use a Profile. The `profile` module provides functionality to create a profile on a data sample (Strings).
Once a profile has been made, data can be generated by calling the _pre_generate()_ and _generate()_ functions, in that order.

```
extern crate test_data_generation;

use test_data_generation::profile::profile::Profile;

fn main() {
    // analyze the dataset
	let mut data_profile =  Profile::new();

    // analyze the dataset
	data_profile.analyze("Smith, John");
	data_profile.analyze("Doe, John");
	data_profile.analyze("Dale, Danny");
	data_profile.analyze("Rickets, Ronney");

    // confirm 4 data samples were analyzed   		
   	assert_eq!(data_profile.patterns.len(), 4);

    // prepare the generator
    data_profile.pre_generate();

    // generate some data
   	println!("The generated name is {:?}", data_profile.generate());

   	// save the profile (algorithm) for later
   	assert_eq!(data_profile.save(&String::from("./tests/samples/sample-00-profile")).unwrap(), true);

   	// later... create a new profile from the saved archive file
   	let mut new_profile = Profile::from_file(&String::from("./tests/samples/sample-00-profile"));
    new_profile.pre_generate();

    // generate some data
   	println!("The generated name is {:?}", new_profile.generate());
}
```

### Data Sample Parser

If you are using CSV files of data samples, then you may wish to use a Data Sample Parser.
The `data_sample_parser` module provides functionality to read sample data, parse and analyze it, so that test data can be generated based on profiles.

```
extern crate test_data_generation;
use test_data_generation::data_sample_parser::DataSampleParser;

fn main() {
    let mut dsp = DataSampleParser::new();
    dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap();

    println!("My new name is {} {}", dsp.generate_record()[0], dsp.generate_record()[1]);
    // My new name is Abbon Aady
}
```

You can also save the Data Sample Parser (the algorithm) as an archive file (json) ...

```
extern crate test_data_generation;
use test_data_generation::data_sample_parser::DataSampleParser;

fn main() {
    let mut dsp =  DataSampleParser::new();  
    dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap();

    assert_eq!(dsp.save(&String::from("./tests/samples/sample-01-dsp")).unwrap(), true);
}
```

and use it at a later time.

```
extern crate test_data_generation;
use test_data_generation::data_sample_parser::DataSampleParser;

fn main() {
    let mut dsp = DataSampleParser::from_file(&String::from("./tests/samples/sample-01-dsp"));

	println!("Sample data is {:?}", dsp.generate_record()[0]);
}
```

You can also generate a new csv file based on the data sample provided.

```
extern crate test_data_generation;
use test_data_generation::data_sample_parser::DataSampleParser;

fn main() {
    let mut dsp =  DataSampleParser::new();  

  	dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap();
    dsp.generate_csv(100, &String::from("./tests/samples/generated-01.csv")).unwrap();
}
```

## How to Contribute

Details on how to contribute can be found in the [CONTRIBUTING](./CONTRIBUTING.md) file.

## License

test-data-generation is primarily distributed under the terms of the Apache License (Version 2.0).

See ![LICENSE-APACHE "Apache License](./LICENSE-APACHE) for details.
