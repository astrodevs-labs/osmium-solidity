const fs = require('fs');

const outputFolder = __dirname + '/../vscode/dist';

fs.readdir(`sidebar/dist`, { withFileTypes: true }, (err, entries) => {
  if (err) {
    console.error(err);
    return;
  }
  const files = entries.filter((file) => file.isFile()).map((file) => file.name);

  files.forEach((file) => {
    console.log('Copying front file to vscode/dist', file);
    fs.copyFile(`sidebar/dist/${file}`, `${outputFolder}/${file}`, (err) => {
      if (err) {
        console.error(err);
      }
    });
  });
});
