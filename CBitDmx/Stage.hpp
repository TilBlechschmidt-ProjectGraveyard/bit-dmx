//
//  Stage.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef Stage_hpp
#define Stage_hpp

class Stage;

#include <stdio.h>
#include <vector>
#include <memory>

#include "Channel.hpp"

#include "UIControlElement.hpp"




class Stage: public sf::Drawable, public sf::Transformable {
public:
    int UIPartWidth = 160;
    Stage(std::string fontPath, std::string stagePath, std::string uiPath);
    
    
    // interacting with the channels
    bool setValue(ChannelAddress address, ChannelValue value, int uiElementID);
    bool startFade(ChannelAddress address, sf::Time fadeTime, ChannelValue value, FadeCurve fadeCurve, int uiElementID);
    
    
    // interactiong with UI Element
    void activateUIElement(int elementID);
    void deactivateUIElement(int elementID);
    
    // get values
    ChannelValue getValue(ChannelAddress address) const;
    bool inEditMode();
    sf::Font getFont();
    sf::Time getNow();
    std::string getName();
    int getChannel(std::string channelName);
    std::vector<int> getChannels(std::vector<std::string> channelNames);
    
    // configrue
    int addUiElement(std::shared_ptr<UIControlElement> uiElement);
    void setName(std::string name);
    
    // other
    void setCurrentTime(sf::Time currentTime);
    bool updateAllChannels();
    
    // events
    void onMousePress(int x, int y, sf::Mouse::Button mouseButton);
    void onMouseMove(int x, int y);
    void onMouseRelease(int x, int y, sf::Mouse::Button mouseButton);
    void onHotkey(sf::Keyboard::Key key);
    void onHotkeyRelease(sf::Keyboard::Key key);
    
    
   

private:
    // Edit mode
    bool m_editMode;
    int m_mouseX;
    int m_mouseY;
    int m_uiElementInEditMode;
    
    //mouse
    int m_lastClickOn;
    sf::Mouse::Button m_lastClickButton;
    
    // other
    sf::Time m_currentTime;
    sf::Font m_font;
    std::string m_name;
    
    //stage data
    std::vector<Channel> m_channels;
    std::vector<std::shared_ptr<UIControlElement>> m_ui_elements;
    json m_namedChannels;

    
    bool updateChannel(ChannelAddress address);
    int findUIElementByXY(int x, int y);
    void toggleEditMode();
    
    virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;
};

#endif /* Stage_hpp */
