import * as fs from 'fs-extra';
import * as path from 'path';
import chalk from 'chalk';
import ora from 'ora';

interface CreateOptions {
  template: string;
}

export async function createProject(projectName: string, options: CreateOptions): Promise<void> {
  const spinner = ora('Creating Stylus project...').start();

  try {
    // Validate project name
    if (!projectName || projectName.length === 0) {
      spinner.fail('Project name is required');
      return;
    }

    // Check if directory already exists
    const projectPath = path.join(process.cwd(), projectName);
    if (fs.existsSync(projectPath)) {
      spinner.fail(`Directory ${projectName} already exists`);
      return;
    }

    // Get template path
    const templatePath = path.join(__dirname, '../../..', 'templates', 'erc20-token');
    
    if (!fs.existsSync(templatePath)) {
      spinner.fail(`Template ${options.template} not found`);
      return;
    }

    spinner.text = 'Copying template files...';

    // Copy template
    await fs.copy(templatePath, projectPath, {
      filter: (src) => {
        // Exclude build artifacts and git
        const basename = path.basename(src);
        return !['target', 'node_modules', '.git'].includes(basename);
      }
    });

    spinner.text = 'Updating project configuration...';

    // Update Cargo.toml with new project name
    const cargoTomlPath = path.join(projectPath, 'Cargo.toml');
    let cargoContent = await fs.readFile(cargoTomlPath, 'utf-8');
    cargoContent = cargoContent.replace(/name = "erc20-token"/, `name = "${projectName}"`);
    await fs.writeFile(cargoTomlPath, cargoContent);

    spinner.succeed(chalk.green('✨ Project created successfully!'));

    // Print next steps
    console.log(chalk.blue('\n📋 Next steps:\n'));
    console.log(chalk.white(`  cd ${projectName}`));
    console.log(chalk.white('  cargo build --release --target wasm32-unknown-unknown'));
    console.log(chalk.white('  cargo stylus check'));
    console.log(chalk.blue('\n📖 Read README.md for more information\n'));

  } catch (error) {
    spinner.fail('Failed to create project');
    console.error(chalk.red(error));
  }
}
