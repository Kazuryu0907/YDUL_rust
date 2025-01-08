import { useState, useEffect } from 'react';
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { UnlistenFn } from "@tauri-apps/api/event";
import { useNavigate } from 'react-router-dom';

const SelectForm = ({cameraNames,setDeviceIndex,disabled}:{cameraNames:string[],setDeviceIndex:React.Dispatch<React.SetStateAction<number>>,disabled:boolean}) => {
  return (
    <div>
      <form className='max-w-sm mx-auto'>
        <label htmlFor="cameras" className='block mb-2 text-sm font-medium text-gray-900'>Select a camera</label>
        <select disabled={disabled} onChange={(e) => setDeviceIndex(Number(e.currentTarget.value))} id="cameras" className='bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5'>
          {cameraNames.map((cameraName, index) => <option value={index} key={cameraName}>{cameraName}</option>)}
        </select>
      </form>
    </div>
    )
}

const WebcamCapture = () => {
  const [deviceIndex, setDeviceIndex] = useState<number>(0);
  const [devices, setDevices] = useState<number[]>([]);
  const [isDetecting,setIsDetecting] = useState<boolean>(true);
  const navigate = useNavigate();

  useEffect(() => {
      // detect_cameras実行
      invoke("detect_cameras");
      let unlisten:UnlistenFn = () => {};
      listen<string>("opencv",(event) => {
          let json = JSON.parse(event.payload);
          console.log(json);
          // detect_camerasの結果をstateに反映
          if(json.cmd === "detect_cameras"){
            let cameras:number[] = json.cameras;
            // pendingを解除
            setIsDetecting(false);
            setDevices(cameras);
          }
      }).then(_unlisten => unlisten = _unlisten);
      return () => {
        unlisten();
      }
    }, []);

  // const currentDevice = devices.length == 0 ? devices[0] : devices[deviceIndex];
  const preview_onclick = () => {
    invoke("preview_camera", {index: devices[deviceIndex]});
  }
  const submit_onclick = () => {
    invoke("set_camera_index", {index: devices[deviceIndex]})
    .then(() => 
      navigate("/monitor")
    )

  }
  const deviceNames = devices.map(index => `Camera ${index}`);
  console.log(devices);
  return (
    <div className='mt-[30vh]'>
        <SelectForm cameraNames={deviceNames} setDeviceIndex={setDeviceIndex} disabled={isDetecting}/>
        <div className='flex mt-10 justify-center'>
          <button className='focus:outline-none text-white bg-green-700 hover:bg-green-800 focus:ring-4 focus:ring-green-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800'
                  onClick={preview_onclick}>Preview</button>
          <button className='focus:outline-none text-white bg-green-700 hover:bg-green-800 focus:ring-4 focus:ring-green-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800'
                  onClick={submit_onclick}>Submit</button>
            {/* {currentDevice != undefined && <Webcam audio={false} videoConstraints={{ deviceId: currentDevice.deviceId }} />} */}
            {/* <Webcam audio={false} videoConstraints={{deviceId: currentDevice.deviceId}} /> */}
            {/* {currentDevice != undefined && currentDevice.toString() || `Device ${deviceIndex + 1}`} */}
            {/* {`DeviceId: ${currentDevice.deviceId}`} */}
        </div>
    </div>
  );
};
export default WebcamCapture;