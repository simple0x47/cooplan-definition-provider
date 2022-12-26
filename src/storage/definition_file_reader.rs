use cooplan_definition_git_downloader::version_detector::VersionDetector;
use cooplan_definitions_io_lib::category_file_io::build_for_all_categories;
use cooplan_definitions_lib::{
    definition::Definition, validated_source_category::ValidatedSourceCategory,
};

use crate::error::{Error, ErrorKind};

pub struct DefinitionFileReader {
    path: String,
}

impl DefinitionFileReader {
    pub fn new(path: String) -> DefinitionFileReader {
        DefinitionFileReader { path }
    }

    pub fn read(&self) -> Result<Definition, Error> {
        let definition = match build_for_all_categories(self.path.clone()) {
            Ok(categories_io) => {
                let mut categories: Vec<ValidatedSourceCategory> = Vec::new();

                for mut category_io in categories_io {
                    match category_io.read() {
                        Ok(source_category) => {
                            match ValidatedSourceCategory::try_from(source_category) {
                                Ok(category) => categories.push(category),
                                Err(error) => {
                                    return Err(Error::new(
                                        ErrorKind::InternalFailure,
                                        format!("failed to validate source category: {}", error),
                                    ));
                                }
                            }
                        }
                        Err(error) => {
                            return Err(Error::new(
                                ErrorKind::InternalFailure,
                                format!("failed to read category: {}", error),
                            ));
                        }
                    }
                }

                let version_detector = VersionDetector::new(self.path.clone());

                match version_detector.read_version() {
                    Ok(version) => {
                        log::info!("version detected: {}", version);
                        let definition = Definition::new(version, categories);

                        definition
                    }
                    Err(error) => {
                        return Err(Error::new(
                            ErrorKind::InternalFailure,
                            format!("failed to read definition's version: {}", error),
                        ));
                    }
                }
            }
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::InternalFailure,
                    format!("failed to read category: {}", error),
                ))
            }
        };

        Ok(definition)
    }
}
