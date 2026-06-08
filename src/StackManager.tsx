import { createContext, ReactNode, useContext, useState } from "react";
import Header from "./Header";

interface StackFunctions {
  addStack: (node: ReactNode) => void;
  closeStack: () => void;
  replaceStack: (node: ReactNode) => void;
  clearStacks: () => void;
}

const StackContext = createContext<StackFunctions | null>(null);

export default function StackManager({ children }: { children: ReactNode }) {
  const [stacks, setStacks] = useState<ReactNode[]>([]);

  const value: StackFunctions = {
    addStack: (node: ReactNode) => {
      setStacks([...stacks, node]);
    },
    closeStack: () => {
      const newStack = [...stacks];
      newStack.splice(-1, 1);
      setStacks(newStack);
    },
    replaceStack: (node: ReactNode) => {
      const newStack = [...stacks];
      newStack.splice(-1, 1, node);
      setStacks(newStack);
    },
    clearStacks: () => setStacks([]),
  };

  return (
    <StackContext.Provider value={value}>
      <div className="flex h-screen flex-col overflow-hidden">
        <Header />
        <div className="px-5 gap-5">
          {stacks.map((stack, i) => (
            <span>Stack #{i + 1}</span>
          ))}
        </div>
        {[children, ...stacks].map((stack, i) => {
          const last = i == stacks.length;
          return <StackLayout hidden={!last}>{stack}</StackLayout>;
        })}
      </div>
    </StackContext.Provider>
  );
}

const hideClassProp = "opacity-0 pointer-events-none absolute inset-0 -z-50 max-h-0 overflow-hidden";
function StackLayout({ children, hidden }: { children: ReactNode; hidden: boolean }) {
  return <div className={"flex-1 min-h-0 flex-row h-full " + (hidden ? hideClassProp : "")}>{children}</div>;
}

export function Stack({ children }: { children: ReactNode }) {
  return children;
}

export const useStacks = () => {
  const context = useContext(StackContext);
  if (!context) {
    throw new Error("No StackContext found.");
  }
  return context;
};
