#!/usr/bin/env node

import { Command } from 'commander';
import chalk from 'chalk';

const program = new Command();

program
  .name('stylus-starter')
  .description('Production-ready Stylus project templates and CLI tool')
  .version('0.1.0');

program
  .command('create <project-name>')
  .description('Create a new Stylus project from a template')
  .option('-t, --template <template>', 'Template to use (erc20, erc721, vault, multisig, upgradeable)', 'erc20')
  .action((projectName: string, options: { template: string }) => {
    console.log(chalk.green(`\n✨ Creating Stylus project: ${projectName}`));
    console.log(chalk.blue(`📋 Template: ${options.template}\n`));
    console.log(chalk.yellow('🚧 Feature coming soon!\n'));
  });

program
  .command('list')
  .description('List available templates')
  .action(() => {
    console.log(chalk.green('\n📋 Available Templates:\n'));
    console.log(chalk.blue('  • erc20       ') + '- ERC20 Token');
    console.log(chalk.blue('  • erc721      ') + '- ERC721 NFT');
    console.log(chalk.blue('  • vault       ') + '- Simple Vault');
    console.log(chalk.blue('  • multisig    ') + '- Multi-sig Wallet');
    console.log(chalk.blue('  • upgradeable ') + '- Upgradeable Contract\n');
  });

program.parse();
