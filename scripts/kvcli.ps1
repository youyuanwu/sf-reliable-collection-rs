param(
    [ValidateSet("Add","Remove","Get", "List")]
    [String]
    $action,
    [String]
    $db = "mystore",
    [String]
    $key,
    [String]
    $val
)
$ErrorActionPreference = "Stop";

# cargo run --bin kvcli -- add --db mystore --key mykey --val myval
if($action -eq "Add") {
    cargo run --bin kvcli -- add --db $db --key $key --val $val   
}elseif ($Action -eq "Remove") {
    cargo run --bin kvcli -- remove --db $db --key $key
}elseif ($Action -eq "Get") {
    cargo run --bin kvcli -- get --db $db --key $key
}elseif ($Action -eq "List") {
    cargo run --bin kvcli -- list --db $db
}