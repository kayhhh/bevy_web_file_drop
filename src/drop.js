const dropped_files = [];

export function init(load_url) {
  const canvas = document.querySelector("canvas");

  canvas.addEventListener("drop", (e) => {
    e.preventDefault();
    e.stopPropagation();

    const file = e.dataTransfer.files[0];
    const ext = file.name.split(".").pop();
    const url = URL.createObjectURL(file);

    dropped_files.push([url, ext]);
  });
}

export function next_dropped_file() {
  return dropped_files.shift();
}
