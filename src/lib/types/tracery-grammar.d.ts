declare module 'tracery-grammar' {
    export interface Grammar {
      flatten: (rule: string) => string;
      addModifiers: (modifiers: Record<string, (s: string) => string>) => void;
    }
  
    export function createGrammar(rules: Record<string, string[] | string>): Grammar;
  
    const tracery: {
      createGrammar: typeof createGrammar;
      baseEngModifiers: Record<string, (s: string) => string>;
    };
  
    export default tracery;
  }