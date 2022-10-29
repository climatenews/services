
interface NewsHeaderProps {
    title: string;
    subtitle: string;
}

export default function NewsHeader(props: NewsHeaderProps) {
    return (
        <>
            <div className="py-2 lg:py-6 bg-gray-200">
                <div className="container px-4 w-full md:max-w-3xl mx-auto ">
                    <h1 className="font-bold font-sans break-normal text-gray-900 text-2xl">
                        {props.title}
                    </h1>
                    <p className="text-sm font-light text-gray-600 mt-1">
                        {props.subtitle}
                    </p>
                </div>
            </div>
        </>
    )
}