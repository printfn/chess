import { Chessground } from 'chessground';
import { Api } from 'chessground/api';
import { Config } from 'chessground/config';
import { useEffect, useRef, useState } from 'react';

interface Props {
	config: Config;
}

export function Board({ config }: Props) {
	const [api, setApi] = useState<Api | null>(null);
	const ref = useRef<HTMLDivElement>(null);

	useEffect(() => {
		if (ref?.current && !api) {
			setApi(Chessground(ref.current, config));
		} else if (ref?.current && api) {
			api.set(config);
		}
	}, [ref]);

	useEffect(() => {
		api?.set(config);
	}, [api, config]);

	return <div ref={ref} style={{ height: '500px', width: '500px' }} />;
}
