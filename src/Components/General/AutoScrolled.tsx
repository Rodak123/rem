import React, { cloneElement, useEffect, useRef, useState } from 'react';
import { cn } from '../../lib/utils';

interface AnimationDataBase {
    scroll: number; // current scroll value
    maxScroll: number; // maximum scroll value
}

interface LoopAnimationData extends AnimationDataBase {
    animationType: 'loop';
    waitTimer: number; // waiting time in seconds
}

interface PingPongAnimationData extends AnimationDataBase {
    animationType: 'ping-pong';
    waitTimer: number; // waiting time in seconds
    direction: number; // 1 - right, -1 - left
}

type AnimationData = LoopAnimationData | PingPongAnimationData;

type AnimationType = AnimationData['animationType'];

interface AutoScrolledProps extends React.ComponentProps<"div"> {
    element: React.ReactElement<React.ComponentPropsWithRef<"div">>; // here I don't know how to make it dynamic, so div will do

    animation: AnimationType;
    waitTimeMs?: number;
}

const ANIMATION_STEP_MS = 10;

export const AutoScrolled: React.FC<AutoScrolledProps> = ({ element, animation, waitTimeMs = 2500, className, ...props }) => {
    const scrollBoxRef = useRef<HTMLDivElement | null>(null); // references the scroll container
    const elementRef = useRef<HTMLDivElement | null>(null); // references the inside element

    const [animationData, setAnimationData] = useState<AnimationData | null>(null);

    // Initialize scrolling
    useEffect(() => {
        if (scrollBoxRef.current === null || elementRef.current === null) return;

        const scrollBox = scrollBoxRef.current;
        const text = elementRef.current;

        const maxScroll = scrollBox.scrollWidth - text.offsetWidth;

        const animationDataBase: AnimationDataBase = {
            scroll: 0,
            maxScroll: maxScroll,
        };

        switch (animation) {
            case 'loop':
                setAnimationData({
                    ...animationDataBase,
                    animationType: animation,
                    waitTimer: 0,
                });
                break;
            case 'ping-pong':
                setAnimationData({
                    ...animationDataBase,
                    animationType: animation,
                    waitTimer: 0,
                    direction: 1,
                });
                break;
        }

    }, [scrollBoxRef, elementRef, animation, waitTimeMs]);

    // Animation loop
    useEffect(() => {
        const loopId = setInterval(() => {
            if (animationData === null) return;

            const waitTimeSec = waitTimeMs / 1000;
            const moveSpeed = 100; // pixels per seconds

            const deltaTime = ANIMATION_STEP_MS / 1000; // seconds between frames

            const nextAnimationData: AnimationData = { ...animationData };

            switch (nextAnimationData.animationType) {
                case 'loop':
                    if (nextAnimationData.waitTimer > 0) {
                        // waiting...
                        nextAnimationData.waitTimer -= deltaTime;

                        if (nextAnimationData.waitTimer <= 0 && nextAnimationData.scroll === nextAnimationData.maxScroll) {
                            // if at end, go to start and wait again :D
                            nextAnimationData.scroll = 0;
                            nextAnimationData.waitTimer = waitTimeSec;
                        }
                        break;
                    }

                    // move
                    nextAnimationData.scroll += (moveSpeed * deltaTime);

                    if (nextAnimationData.scroll > animationData.maxScroll) {
                        nextAnimationData.scroll = animationData.maxScroll;
                        nextAnimationData.waitTimer = waitTimeSec / 2;
                        // wait at end
                    }
                    break;

                case 'ping-pong':
                    if (nextAnimationData.waitTimer > 0) {
                        nextAnimationData.waitTimer -= deltaTime;
                        break;
                    }

                    // move 
                    nextAnimationData.scroll += (moveSpeed * nextAnimationData.direction * deltaTime);

                    if (nextAnimationData.scroll < 0) {
                        nextAnimationData.scroll = 0;
                        nextAnimationData.waitTimer = waitTimeSec / 2;
                        nextAnimationData.direction *= -1;
                        // wait at start and flip direction
                    }

                    if (nextAnimationData.scroll > animationData.maxScroll) {
                        nextAnimationData.scroll = animationData.maxScroll;
                        nextAnimationData.waitTimer = waitTimeSec / 2;
                        nextAnimationData.direction *= -1;
                        // wait at end and flip direction
                    }
                    break;
            }

            setAnimationData(nextAnimationData);
        }, ANIMATION_STEP_MS);

        return () => {
            clearTimeout(loopId);
        };
    }, [animationData, waitTimeMs]);

    // Scroll to target position
    useEffect(() => {
        if (animationData === null || scrollBoxRef.current === null) return;

        scrollBoxRef.current.scrollTo({
            behavior: 'smooth',
            left: animationData.scroll,
        });
    }, [animationData]);

    // Clone the scrolled element and make sure its block and text doesn't wrap
    const clonedElm = cloneElement(element, {
        ref: elementRef,
        className: cn(element.props.className, 'block text-nowrap'),
    });

    return (
        <div
            {...props}
            ref={scrollBoxRef}
            className={cn(
                className,
                "overflow-scroll scrollbar-none"
            )}
        >
            {clonedElm}
        </div>
    );
};