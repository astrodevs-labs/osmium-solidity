const fs = require('fs');

const targetFolder = 'target';
const outputFolder = __dirname + '/../vscode/dist';

fs.readdir('target', { withFileTypes: true }, (err, dirs) => {
  if (err) {
    console.error(err);
    return;
  }

  console.log('Target entries: ', dirs);

  const directories = dirs.filter(file => file.isDirectory()).map(file => file.name);
  const targetDirectories = directories.filter(directory => directory == "release" || directory == "debug");
  const dir = targetDirectories.find(dir => dir == "release") || targetDirectories[0];
  fs.readdir(`target/${dir}`, { withFileTypes: true }, (err, entries) => {
    if (err) {
        console.error(err);
        return;
      }
    const files = entries.filter(file => file.isFile()).map(file => file.name);
    const serverBinaries = files.filter(file => file.endsWith('-server') || file.endsWith('-server.exe'));
    console.log('Copying server binaries to vscode/dist', serverBinaries);
    serverBinaries.forEach(binary => {
        fs.copyFile(`target/${dir}/${binary}`, `${outputFolder}/${binary}`, (err) => {
            if (err) {
            console.error(err);
            }
        });
    });
  });
});
