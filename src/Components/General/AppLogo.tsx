import { cn } from '../../lib/utils';

interface AppLogoProps extends React.HTMLAttributes<HTMLDivElement> { }

export const AppLogo: React.FC<AppLogoProps> = ({ ...props }) => {
    const { className } = props;

    return (
        <div
            {...props}
            className={cn(className, 'aspect-square bg-white')}
            title="REM"
        >
            { /* The app logo/icon */}
        </div>
    );
};