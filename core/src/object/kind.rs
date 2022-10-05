use std::{
	fmt::{Display, Formatter},
	str::FromStr,
};

use int_enum::IntEnum;
use rspc::Type;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Type, IntEnum)]
#[repr(u8)]
pub enum ObjectKind {
	// A file that can not be identified by the indexer
	Unknown = 0,
	// A known filetype, but without specific support
	Document = 1,
	// A virtual filesystem directory
	Folder = 2,
	// A file that contains human-readable text
	Text = 3,
	// A virtual directory int
	Package = 4,
	// An image file
	Image = 5,
	// An audio file
	Audio = 6,
	// A video file
	Video = 7,
	// A compressed archive of data
	Archive = 8,
	// An executable, program or application
	Executable = 9,
	// A link to another object
	Alias = 10,
	// Raw bytes encrypted by Spacedrive with self contained metadata
	Encrypted = 11,
	// A link can open web pages, apps or Spaces
	Key = 12,
	// A link can open web pages, apps or Spaces
	Link = 13,
	// A special filetype that represents a preserved webpage
	WebPageArchive = 14,
	// A widget is a mini app that can be placed in a Space at various sizes, associated Widget struct required
	Widget = 15,
	// Albums can only have one level of children, and are associated with the Album struct
	Album = 16,
	// Its like a folder, but appears like a stack of files, designed for burst photos / associated groups of files
	Collection = 17,
}

macro_rules! extension_enum {
	(
		Extension {
			$( $variant:ident($type:ident), )*
		}
	) => {
		#[derive(Debug, Serialize, Deserialize)]
		pub enum Extension {
			$( $variant($type), )*
		}
		// convert extension to object kind
		impl From<Extension> for ObjectKind {
			fn from(ext: Extension) -> Self {
				match ext {
					$( Extension::$variant(_) => ObjectKind::$variant, )*
				}
			}
		}
		impl Display for Extension {
			fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
				match self {
					$( Extension::$variant(x) =>  write!(f, "{}", x), )*
				}
			}
		}
	};
}

extension_enum! {
	Extension {
		Unknown(String),
		Video(VideoExtension),
		Image(ImageExtension),
		Audio(AudioExtension),
		Archive(ArchiveExtension),
		Executable(ExecutableExtension),
		Text(TextExtension),
		Encrypted(EncryptedExtension),
		Key(KeyExtension),
	}
}

/// Define a public enum with static array of all possible variants
/// including implementations to convert to/from string
macro_rules! enum_with_variants {
	(
		$(#[$enum_attr:meta])*
		$enum_name:ident $static_array_name:ident {
			$($(#[$variant_attr:meta])* $variant:ident, )*
		}
	) => {
		#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
		#[serde(rename_all = "snake_case")]
		$(#[$enum_attr])*
		// construct enum
		pub enum $enum_name {
			$( $(#[$variant_attr])* $variant, )*
		}
		// a static array of all variants
		pub static $static_array_name: &[$enum_name] = &[
			$( $enum_name::$variant, )*
		];
		// convert from string
		impl FromStr for $enum_name {
			type Err = serde_json::Error;
			fn from_str(s: &str) -> Result<Self, Self::Err> {
				serde_json::from_value(Value::String(s.to_string()))
			}
		}
		// convert to string
		impl std::fmt::Display for $enum_name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{}", serde_json::to_string(self).unwrap()) // SAFETY: This is safe
			}
		}
	}
}

// video extensions
enum_with_variants! {
	VideoExtension ALL_VIDEO_EXTENSIONS {
		Avi,
		Asf,
		Mpeg,
		Mts,
		Mpg,
		Mpe,
		Qt,
		Mov,
		Swf,
		Mjpeg,
		Ts,
		Mxf,
		M2v,
		M2ts,
		Flv,
		Wm,
		#[serde(rename = "3gp")]
		_3gp,
		M4v,
		Wmv,
		Mp4,
		Webm,
	}
}

// image extensions
enum_with_variants! {
	ImageExtension _ALL_IMAGE_EXTENSIONS {
		Jpg,
		Jpeg,
		Png,
		Gif,
		Bmp,
		Tiff,
		Webp,
		Svg,
		Ico,
		Heic,
	}
}

// audio extensions
enum_with_variants! {
	AudioExtension _ALL_AUDIO_EXTENSIONS {
		Mp3,
		M4a,
		Wav,
		Aiff,
		Aif,
		Flac,
		Ogg,
		Opus,
	}
}

// archive extensions
enum_with_variants! {
	ArchiveExtension _ALL_ARCHIVE_EXTENSIONS {
		Zip,
		Rar,
		Tar,
		Gz,
		Bz2,
		SevenZip,
	}
}

// executable extensions
enum_with_variants! {
	ExecutableExtension _ALL_EXECUTABLE_EXTENSIONS {
		Exe,
		App,
		Apk,
		Deb,
		Dmg,
		Pkg,
		Rpm,
		Msi,
	}
}

// document extensions
enum_with_variants! {
	DocumentExtension _ALL_DOCUMENT_EXTENSIONS {
		Pdf,
		Key,
		Pages,
		Numbers,
		Doc,
		Docx,
		Xls,
		Xlsx,
		Ppt,
		Pptx,
		Odt,
		Ods,
		Odp,
		Ics,
	}
}

// text file extensions
enum_with_variants! {
	TextExtension _ALL_TEXT_EXTENSIONS {
		Txt,
		Rtf,
		Csv,
		Html,
		Css,
		Json,
		Yaml,
		Xml,
		Md,
	}
}
// encrypted file extensions
enum_with_variants! {
	EncryptedExtension _ALL_ENCRYPTED_EXTENSIONS {
		Bit,	// Spacedrive encrypted file
		Box,	// Spacedrive container
		Block,// Spacedrive block storage,
	}
}

// key extensions
enum_with_variants! {
	KeyExtension _ALL_KEY_EXTENSIONS {
		Pgp,
		Pub,
		Pem,
		P12,
		P8,
		Keychain,
	}
}