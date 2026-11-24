mod body;
mod scripts;
mod styles;

use body::BODY;
use scripts::SCRIPTS;
use styles::STYLES;

pub fn get_index_html() -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <title>Daily Checksheet & Evaluation Test</title>
    <style>{STYLES}</style>
</head>
<body>{BODY}
    <script>{SCRIPTS}</script>
</body>
</html>
"#
    )
}
