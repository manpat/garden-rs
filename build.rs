use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const INDEX_HTML_TEMPLATE: &'static str = 
r##"<html>
	<head>
		<meta name='viewport' content='width=device-width, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0, user-scalable=no' />
		<meta name="apple-mobile-web-app-capable" content="yes">
		<meta name="mobile-web-app-capable" content="yes">

		<meta name="theme-color" content="#333">
		<meta name="msapplication-navbutton-color" content="#333">
		<meta name="apple-mobile-web-app-status-bar-style" content="#333">

		<style>
			* {
				margin: 0;
				padding: 0;
			}

			html, body {
				width: 100vw;
				height: 100vh;
				overflow: hidden;
			}

			canvas {
				overflow: hidden;
				display: block;
			}
		</style>
	</head>

	<body>
		<canvas id="canvas"></canvas>
		<script src="target/asmjs-unknown-emscripten/[[build_type]]/[[pkg_name]].js"></script>
	</body>
</html>"##;

fn main() {
	let index_html = INDEX_HTML_TEMPLATE.to_string()
		.replace("[[build_type]]", &env::var("PROFILE").unwrap())
		.replace("[[pkg_name]]", env!("CARGO_PKG_NAME"));
	let dest = env::var("CARGO_MANIFEST_DIR").unwrap();
	let path = Path::new(&dest).join("index.html");
	let mut file = File::create(&path).unwrap();

	file.write_all(index_html.as_bytes()).unwrap();
}