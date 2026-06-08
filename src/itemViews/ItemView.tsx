import { useSelector } from "react-redux";
import { RootState } from "../store/store";

export default function ItemView() {
  const openItem = useSelector((state: RootState) => state.workspace.openItem);

  const renderItem = () => {
    switch (openItem?.type) {
      case "photo":
        return <></>;
      case "raw":
      // return <RawView id={openItem.id} />;
      case "species":
        return <></>;
    }
  };

  return <div>{renderItem()}</div>;
}
