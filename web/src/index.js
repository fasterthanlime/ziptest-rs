require("file-loader?name=[name].[ext]!./index.html");

async function main() {
  const pkg = await import("../../pkg");

  const fileInput = document.querySelector("input[type=file]");
  fileInput.addEventListener("change", (ev) => {
    pkg.list_files(fileInput.files);
  });
  document.querySelector("#loader").remove();
  console.log("Ready!");
}

main();

