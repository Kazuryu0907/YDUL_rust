import { useState, useCallback, useEffect } from 'react';
import Webcam from 'react-webcam';
import { invoke } from "@tauri-apps/api/core";

const View = () => {
    const [camId, setCamId] = useState<number>(0);
    const onclick = () => {
        invoke("camera_preview", {index: camId})
    }
    return (
        <div>
            View
            <h1>{camId}</h1>
            <div>
                <button>+</button>
                <button onClick={onclick}>Preview</button>
                <button>-</button>
            </div>
        </div>
    );
}

const SelectForm = ({cameraNames,setDeviceIndex}:{cameraNames:string[],setDeviceIndex:React.Dispatch<React.SetStateAction<number>>}) => {
  return (
    <div>
      <form className='max-w-sm mx-auto'>
        <label htmlFor="cameras" className='block mb-2 text-sm font-medium text-gray-900'>Select a camera</label>
        <select onChange={(e) => setDeviceIndex(Number(e.currentTarget.value))} id="cameras" className='bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5'>
          {cameraNames.map((cameraName, index) => <option value={index}>{cameraName}</option>)}
        </select>
      </form>
    </div>
    )
}

const WebcamCapture = () => {
  const [deviceIndex, setDeviceIndex] = useState<number>(0);
  const [devices, setDevices] = useState<MediaDeviceInfo[]>([]);

  const handleDevices:(mediaDevices:MediaDeviceInfo[]) => void = useCallback(
    mediaDevices =>
      setDevices(mediaDevices.filter(({ kind }) => kind === "videoinput")),
    [setDevices]
  );

  useEffect(
    () => {
      navigator.mediaDevices.enumerateDevices().then(handleDevices);
    },
    [handleDevices]
  );

  const currentDevice = devices.length == 0 ? devices[0] : devices[deviceIndex];
  const deviceNames = devices.map(device => device.label);
  console.log(devices);
  return (
    <>
        <SelectForm cameraNames={deviceNames} setDeviceIndex={setDeviceIndex}/>
        <div>
            {currentDevice != undefined && <Webcam audio={false} videoConstraints={{ deviceId: currentDevice.deviceId }} />}
            {/* <Webcam audio={false} videoConstraints={{deviceId: currentDevice.deviceId}} /> */}
            {currentDevice != undefined && currentDevice.label || `Device ${deviceIndex + 1}`}
            {`DeviceId: ${currentDevice.deviceId}`}
        </div>
    </>
  );
};
export default View;