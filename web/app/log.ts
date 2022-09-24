export const log = config => (set: any, get: any, api: any) => config(args => {
    console.log("  applying", args)
    set(args)
    // console.log("  new state", get())
}, get, api)
