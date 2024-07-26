const dropped_files = [];

export function init() {
  const canvas = document.querySelector("canvas");

  canvas.addEventListener("dragover", (e) => {
    e.preventDefault();
  });

  canvas.addEventListener("dragenter", (e) => {
    e.preventDefault();
  });

  canvas.addEventListener("drop", (e) => {
    e.preventDefault();

    if (!e.dataTransfer || e.dataTransfer.files.length == 0) {
      console.warn("No files dropped");
      return;
    }

    const files = e.dataTransfer.files;
    for(let i = 0; i < files.length; i++) {
      const file = files[i];
      const ext = file.name.split(".").pop();
      const url = URL.createObjectURL(file);

      dropped_files.push([url, ext]);
    }
  });
}

export function next_dropped_file() {
  return dropped_files.shift();
}
