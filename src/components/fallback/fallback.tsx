import { Grid } from "react-loader-spinner";

function Fallback() {
  return (
    <div className="flex-1 flex items-center justify-center">
      <Grid 
        color="#cf8d23" 
        width={32} 
        height={32} 
        wrapperClass="shrink-0" 
        visible={true}
      />
    </div>
  )
}

export default Fallback;
