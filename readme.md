# File Organizer

This Rust project helps you organize any directory on your system by sorting files into subdirectories based on their file types. It can also revert the organization and move all files back to the main directory.

## Features

- **Organize**: Organizes a specified directory into subdirectories based on file types. It monitors the directory and automatically moves new files to their respective directories. Initiate this feature by selecting "Organize" from the main menu.
- **Revert**: Reverts the organization done by the "Organize" feature. Moves all files back to the main directory. Initiate this feature by selecting "Revert" from the main menu.

## Usage

1. Run the program.
2. Enter the path of the directory you want to organize when prompted.
3. Select an action from the menu: "Organize", "Revert", or "Exit".
4. If you select "Organize" or "Revert", the program will start monitoring the specified directory. It will either organize new files or revert the organization, respectively.
5. To stop the "Organize" or "Revert" function and return to the main menu, simply write "stop".

## Building

To build this project, you need to have Rust installed on your machine. 
- Clone code with ```git clone https://github.com/jekabso21/download_organizer.git```
- Use ```cargo build``` to build the project.

## Contributing

Contributions are welcome. Please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).