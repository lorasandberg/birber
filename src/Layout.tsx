import { Spinner } from "#components/ui/spinner";
import { useLoading } from "#lib/utils";
import { useSelector } from "react-redux";
import PhotoBrowser from "./PhotoBrowser";
import StackManager from "./StackManager";
import { RootState } from "./store/store";

export default function Layout() {
  const openItem = useSelector((state: RootState) => state.workspace.openItem);

  const loading: boolean = useLoading();

  if (loading)
    return (
      <div className="flex justify-center items-center min-h-screen overflow-hidden">
        <Spinner className="size-24" />
      </div>
    );

  return (
    <StackManager>
      <PhotoBrowser />
    </StackManager>
  );
}
