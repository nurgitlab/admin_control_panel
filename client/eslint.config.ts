import js from '@eslint/js';
import typescriptEslint from 'typescript-eslint';
import react from 'eslint-plugin-react';
import reactHooks from 'eslint-plugin-react-hooks';
import importPlugin from 'eslint-plugin-import';
import jsxA11y from 'eslint-plugin-jsx-a11y';
import prettier from 'eslint-config-prettier';
import globals from 'globals';

export default [
  { 
    ignores: ['dist', '.eslintrc.cjs', 'eslint.config.ts', 'node_modules/'] 
  },
  js.configs.recommended,
  ...typescriptEslint.configs.recommended,
  
  {
    files: ['**/*.{js,jsx,ts,tsx}'],
    plugins: { 
      react, 
      'react-hooks': reactHooks,
      'jsx-a11y': jsxA11y,
      'import': importPlugin
    },
    rules: {
      ...react.configs.recommended.rules,
      ...reactHooks.configs.recommended.rules,
      ...jsxA11y.configs.recommended.rules,
      
      // React правила
      'react/react-in-jsx-scope': 'off',
      'react/jsx-props-no-spreading': 'off',
      'react/no-array-index-key': 'off',
      'react/destructuring-assignment': 'off',
      'react/require-default-props': 'off',
      
      // JSX-A11y правила
      'jsx-a11y/click-events-have-key-events': 'off',
      'jsx-a11y/no-static-element-interactions': 'off',
      'jsx-a11y/anchor-is-valid': 'off',
      
      // Import правила (временно отключаем проблемы с резолвингом)
      'import/no-unresolved': 'off',
      'import/named': 'off',
      'import/namespace': 'off',
      'import/default': 'off',
      'import/prefer-default-export': 'off',
      'import/extensions': 'off',
      'import/no-extraneous-dependencies': 'off',
      
      // Общие правила
      'no-restricted-syntax': ['off', 'ForOfStatement'],
      'no-console': ['error', { allow: ['warn', 'error'] }],
    },
    settings: { 
      react: { version: 'detect' } 
    },
    languageOptions: {
      globals: { 
        ...globals.browser,
        ...globals.node 
      },
      parserOptions: {
        ecmaVersion: 2020,
        sourceType: 'module',
        ecmaFeatures: { jsx: true },
        project: './tsconfig.json'
      },
    },
  },
  
  {
    files: ['**/*.{js,jsx,ts,tsx}'],
    rules: {
      // Кастомные правила импортов
      'import/order': "off",
      
      // Кастомные React правила
      'react/function-component-definition': [
        'error',
        {
          namedComponents: 'arrow-function',
          unnamedComponents: 'arrow-function',
        },
      ],
      'react-hooks/exhaustive-deps': 'error',
      
      // Кастомные TypeScript правила
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/explicit-function-return-type': 'off', // Меняем на off, так как это слишком строго
      '@typescript-eslint/no-unused-vars': 'error',
    },
  },
  
  // Prettier конфигурация
  prettier,
];