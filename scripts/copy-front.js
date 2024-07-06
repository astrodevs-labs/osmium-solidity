const fs = require('fs');

const outputFolder = __dirname + '/../vscode/dist';
const projects = ['sidebar', 'env-panel'];

for (const project of projects) {
  fs.readdir(`${project}/dist`, { withFileTypes: true }, (err, entries) => {
    if (err) {
      console.error(err);
      return;
    }
    const files = entries
      .filter((file) => file.isFile())
      .map((file) => file.name)
      .filter((file) => file !== 'index.html');

    files.forEach((file) => {
      console.log('Copying front file to vscode/dist', file);
      fs.copyFile(`${project}/dist/${file}`, `${outputFolder}/${file}`, (err) => {
        if (err) {
          console.error(err);
        }
      });
    });
  });
}
