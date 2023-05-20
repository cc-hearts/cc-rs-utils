```ts
{
  "compilerOptions": {
    "target": "ESNext",
    // "outDir": "src/lib/esm",
    "lib": ["ESNext"],
    // 启用装饰器
    "experimentalDecorators": true,
    "emitDecoratorMetadata": true,
    // Modules
    "module": "ESNext",
    "moduleResolution": "node",
    // javaScript not supported
    "allowJs": false,
    // Emit
    "noEmit": false,
    "downlevelIteration": true,
    "importHelpers": true,
    "sourceMap": false,
    "stripInternal": true,
    // Constraints
    "allowSyntheticDefaultImports": true,
    "esModuleInterop": true,
    "forceConsistentCasingInFileNames": true,
    // Type Checking
    "noImplicitThis": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noImplicitReturns": true,
    "noFallthroughCasesInSwitch": true,
    // type dts
    "declaration": true,
    "declarationDir":"./types",
    "emitDeclarationOnly": true
  },
}

```