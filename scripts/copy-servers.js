const fs = require('fs');

const targetFolder = 'target';
const outputFolder = __dirname + '/../vscode/dist';

fs.readdir('target', { withFileTypes: true }, (err, dirs) => {
  if (err) {
    console.error(err);
    return;
  }

  const directories = dirs.filter(file => file.isDirectory()).map(file => file.name);
  const targetDirectories = directories.filter(directory => directory !== 'tmp' && directory !== 'debug');
  const dir = targetDirectories[0];
  fs.readdir(`target/${dir}`, { withFileTypes: true }, (err, entries) => {
    if (err) {
        console.error(err);
        return;
      }
    const files = entries.filter(file => file.isFile()).map(file => file.name);
    const serverBinaries = files.filter(file => file.endsWith('-server') || file.endsWith('-server.exe'));
    serverBinaries.forEach(binary => {
        fs.copyFile(`target/${dir}/${binary}`, `${outputFolder}/${binary}`, (err) => {
            if (err) {
            console.error(err);
            }
        });
    });
  });
});
