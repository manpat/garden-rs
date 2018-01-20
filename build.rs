use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const INDEX_HTML_TEMPLATE: &'static str = 
r##"<html>
	<head>
		<meta charset="utf-8"/>
		<meta name='viewport' content='width=device-width, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0, user-scalable=no' />
		<meta name="apple-mobile-web-app-capable" content="yes" />
		<meta name="mobile-web-app-capable" content="yes" />

		<meta name="theme-color" content="#74ae76"/>
		<meta name="msapplication-navbutton-color" content="#74ae76"/>
		<meta name="apple-mobile-web-app-status-bar-style" content="#74ae76"/>

		<style>
			* {
				margin: 0;
				padding: 0;
				user-select: none;
				-moz-user-select: none;
				-khtml-user-select: none;
				-webkit-user-select: none;
				-o-user-select: none;
			}

			html, body {
				width: 100vw;
				height: 100vh;
				overflow: hidden;
				background: #69cc82;
			}

			canvas {
				overflow: hidden;
				display: block;
			}
		</style>
	</head>

	<body>
		<canvas id="canvas"></canvas>
		<script src="[[pkg_name]]/[[build_type]].js"></script>
	</body>
</html>"##;

fn main() {
	let profile = env::var("PROFILE").unwrap();

	let index_html = INDEX_HTML_TEMPLATE.to_string()
		.replace("[[build_type]]", &profile)
		.replace("[[pkg_name]]", env!("CARGO_PKG_NAME"));
	let dest = env::var("CARGO_MANIFEST_DIR").unwrap();
	let path = Path::new(&dest).join("index.html");
	let mut file = File::create(&path).unwrap();

	file.write_all(index_html.as_bytes()).unwrap();

	if profile == "debug" {
		println!("cargo:rustc-cfg=debug");
	}
}
