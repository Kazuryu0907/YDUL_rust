import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { UnlistenFn } from "@tauri-apps/api/event";
import {convertFileSrc} from "@tauri-apps/api/core";
import ReactPlayer from "react-player";
import {debug} from "@tauri-apps/plugin-log";

const Monitor = () => {
    const [isChecked, setIsChecked] = useState(false);
    const [videoPath,setVideoPath] = useState<string>("");
    // 0がCompetitive, 1がDeath Match
    const handleCheckboxChange = () => {
        setIsChecked(!isChecked)
    };
    let pre_value = 0;

    let unlisten:UnlistenFn = () => {};
    let unlisten_replay_buffer:UnlistenFn = () => {};

    useEffect(() => {
        // 一個前で設定したカメラIndex取得
        invoke("get_camera_index").then(index => {
            invoke("opencv_watch", {index: index});
        });
    },[]);
    useEffect(() => {
        (async () => {
            const unlisten = await listen<string>("replay_buffer", (event) => {
                debug("replay_buffer " + `${event.payload}`);
                // 一旦前のファイルを削除
                invoke("delete_file",{path:videoPath}).then((e) => debug(e as string)).catch(console.error);
                setVideoPath(event.payload);
            });
            unlisten_replay_buffer = unlisten;
        })();

        // cleanup
        return () => {
            unlisten_replay_buffer();
        }
    },[videoPath]);  

    useEffect(() => {
        debug(`currentIsChecked: ${isChecked}`);
        listen<string>("opencv", (event) => {
          let json = JSON.parse(event.payload);
          if(json.cmd === "watch"){
            let current_mode = isChecked ? "red" : "gray";
            if(json.kind === current_mode){
                if(json.value != pre_value){
                    // Fireしたらここ
                    debug("same:",json.kind);
                    invoke("obs_save_replay_buffer").then(e => debug(e as string)).catch(console.error);
                }
            }
            pre_value = json.value;
          }
        }).then(_unlisten => unlisten = _unlisten);

        return () => {
            unlisten();
        }
    },[isChecked]);

    return(
        <div className="w-auto">
            {/* {isChecked ? "1" : "0"} */}
            <label className='themeSwitcherTwo shadow-card relative inline-flex cursor-pointer select-none items-center justify-center rounded-md bg-white p-1'>
                <input
                type='checkbox'
                className='sr-only'
                checked={isChecked}
                onChange={handleCheckboxChange}
                />
                <span
                className={`flex items-center space-x-[6px] rounded py-2 px-[18px] text-lg font-bold ${
                    !isChecked ? 'text-primary bg-green-500' : 'text-body-color'
                }`}
                >
                Competitive
                </span>
                <span
                className={`flex items-center space-x-[6px] rounded py-2 px-[18px] text-lg font-bold ${
                    isChecked ? 'text-primary bg-green-500' : 'text-body-color'
                }`}
                >
                Death Match
                </span>
            </label>
            <ReactPlayer url={convertFileSrc(videoPath)} width={"auto"} height={"auto"} playing={true} muted={true} playbackRate={0.2} 
            />
        </div>
    )
}


export default Monitor;