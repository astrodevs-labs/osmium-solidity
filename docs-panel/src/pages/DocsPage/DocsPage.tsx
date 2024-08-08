import { ChevronDownIcon } from '@heroicons/react/20/solid';
import { Disclosure, DisclosureButton, DisclosurePanel } from '@headlessui/react'
import './DocsPage.css';

export const DocsPage = () => {
  const features = [
    { name: 'Linter', description: 'Underlines issues in your code.' },
    { name: 'Formatter', description: 'Automatically formats your document.' },
    { name: 'Gas estimation', description: 'Shows gas cost for contract execution.' },
    { name: 'Slither', description: 'Security issue scanner for contracts.' },
    { name: 'Deploy & Interact sidebar', description: 'Interact with and deploy contracts.' },
    { name: 'Debugger', description: 'Debug your smart contracts.' },
    { name: 'Unit tests', description: 'Test your smart contracts with Foundry.' },
    { name: 'Compiler', description: 'Compiles your solidity code.' },
    { name: 'References', description: 'Navigate to usages of variables.' },
    { name: 'Auto format', description: 'Automatically format your code on save.' },
  ];

  function classNames(...classes: string[]) {
    return classes.filter(Boolean).join(' ');
  }

  return (
    <div className="flex">
      <div className="h-screen overflow-y-scroll w-full md:w-3/4">
        <div className="max-w-5xl mx-auto px-4 sm:px-6 md:px-8">
          <h1 className="text-3xl font-extrabold ml-4">Osmium Extension Documentation</h1>
          <div className="space-y-8 md:space-y-16">
            {features.map((feature) => (
              <Disclosure as="div" key={feature.name} className="pt-6">
                {({ open }) => (
                  <>
                    <dt className="text-lg">
                      <DisclosureButton
                        className={classNames(
                          open ? 'text-white-900' : 'text-white-500',
                          'group w-full flex justify-between items-start text-left',
                        )}
                      >
                        <span className="font-medium">{feature.name}</span>
                        <span className="ml-6 flex items-center">
                          <ChevronDownIcon
                            className={classNames(
                              open ? 'text-white-600' : 'text-white-400',
                              'h-5 w-5 group-hover:text-white-500',
                            )}
                            aria-hidden="true"
                          />
                        </span>
                      </DisclosureButton>
                    </dt>
                    <DisclosurePanel as="dd" className="pr-12">
                      <p className="text-base text-white-500">{feature.description}</p>
                    </DisclosurePanel>
                  </>
                )}
              </Disclosure>
            ))}
          </div>
        </div>
      </div>

      <div className="hidden md:block bg-white-200 w-1/4">
        <div className="space-y-4 p-4">
          <h2 className="text-xl font-semibold text-white-900">Table of Contents</h2>
          <nav className="space-y-1">
            {features.map((feature) => (
              <a
                key={feature.name}
                href={`#${feature.name.toLowerCase().replace(' ', '-')}`}
                className="group flex items-center px-2 py-1 text-base font-medium rounded-md text-white-700 hover:bg-white-30 hover:text-white-900"
              >
                <span className="flex-1 ml-2">{feature.name}</span>
              </a>
            ))}
          </nav>
        </div>
      </div>
    </div>
  );
};
