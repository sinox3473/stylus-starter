#!/usr/bin/env node

import { Command } from 'commander';
import chalk from 'chalk';
import { createProject } from './commands/create';

const program = new Command();

program
  .name('stylus-starter')
  .description('Production-ready Stylus project templates and CLI tool')
  .version('0.1.0');

program
  .command('create <project-name>')
  .description('Create a new Stylus project from a template')
  .option('-t, --template <template>', 'Template to use (erc20, erc721, vault, multisig, upgradeable)', 'erc20')
  .action(async (projectName: string, options: { template: string }) => {
    console.log(chalk.green(`\n✨ Creating Stylus project: ${projectName}`));
    console.log(chalk.blue(`📋 Template: ${options.template}\n`));
    
    await createProject(projectName, options);
  });

program
  .command('list')
  .description('List available templates')
  .action(() => {
    console.log(chalk.green('\n📋 Available Templates:\n'));
    console.log(chalk.blue('  • erc20       ') + chalk.white('- ERC20 Token (Production Ready ⭐⭐⭐⭐⭐)'));
    console.log(chalk.gray('  • erc721      ') + chalk.gray('- ERC721 NFT (Coming Soon)'));
    console.log(chalk.gray('  • vault       ') + chalk.gray('- Simple Vault (Coming Soon)'));
    console.log(chalk.gray('  • multisig    ') + chalk.gray('- Multi-sig Wallet (Coming Soon)'));
    console.log(chalk.gray('  • upgradeable ') + chalk.gray('- Upgradeable Contract (Coming Soon)\n'));
  });

program.parse();
